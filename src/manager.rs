use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::{Duration, Instant};

use gdjs::util::handle_error;
use godot::classes::control::LayoutPreset;
use godot::classes::{CanvasLayer, Label};
use js_core::timers::JsTimers;

use crate::bifrost_js_module::BifrostModule;
use crate::node::create_godot_js_proxy;
pub use crate::prelude::*;
use crate::proxy_deps::ProxyDeps;

pub struct JsNodeBound {
    pub js_object: js::Persistent<js::Object<'static>>,
}

struct DeadlineGuard {
    cell: Rc<Cell<Option<Instant>>>,
}
impl DeadlineGuard {
    fn new(deadline: Rc<Cell<Option<Instant>>>, limit: u64) -> Self {
        if limit > 0 {
            deadline.set(Some(Instant::now() + Duration::from_millis(limit)));
        }
        Self { cell: deadline }
    }
}
impl Drop for DeadlineGuard {
    fn drop(&mut self) {
        self.cell.set(None);
    }
}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct JsRuntimeManager {
    base: Base<Node>,
    js_runtime: Option<js::Runtime>,
    js_context: Option<js::Context>,
    bounds: HashMap<InstanceId, JsNodeBound>,
    process_queue: Vec<InstanceId>,
    ready_queue: Vec<Gd<Node>>,
    free_queue: Vec<JsNodeBound>,
    context: ManagerCtxRef,
    deadline: Rc<Cell<Option<Instant>>>,
    timers: JsTimers,

    #[export]
    execution_limit: u32, // in ms where 0 is unlimited
    #[export]
    debug_enabled: bool,
    start_time: Instant,
    debug_label: Option<Gd<Label>>,
    last_debug_update: f64,
}

#[godot_api]
impl JsRuntimeManager {
    fn ensure_initialized(&mut self) {
        if self.js_context.is_some() {
            return;
        }

        godot_print!("--- JS Engine initialization.. ---");

        let runtime = js::Runtime::new().expect("JS Runtime create failure");
        runtime.set_loader(gdjs::modules::JsResolver, gdjs::modules::JsLoader);
        if self.execution_limit > 0 {
            let deadline = self.deadline.clone();
            runtime.set_interrupt_handler(Some(Box::new(move || {
                let Some(deadline) = deadline.get() else {
                    return false;
                };
                deadline - Instant::now() <= Duration::from_millis(0)
            })));
        }

        let context = js::Context::full(&runtime).expect("JS Context failure");

        let manager_ctx = self.context.clone();
        let timers = self.timers.clone();
        context.with(move |ctx| {
            let res = (|| -> js::Result<()> {
                let globals = ctx.globals();
                globals.set("console", gdjs::console::create(&ctx))?;
                timers.bind_to(&ctx, &globals)?;
                ctx.store_userdata(manager_ctx)?;
                js::Module::declare_def::<BifrostModule, _>(ctx.clone(), "bifrostjs")?;
                Ok(())
            })();
            gdjs::util::handle_error(&ctx, &res);
        });

        self.js_runtime = Some(runtime);
        self.js_context = Some(context);
    }

    fn ctx(&self) -> &js::Context {
        self.js_context.as_ref().expect("Context must be inited")
    }

    #[func]
    fn register_js_node(&mut self, node: Gd<Node>, path: GString) {
        self.register_node(node, path.to_string());
    }

    fn register_node(&mut self, gd_node: Gd<Node>, script_path: String) {
        let instance_id = gd_node.instance_id();

        let file = match gdjs::util::load_ts(&script_path) {
            Ok(f) => f,
            Err(m) => {
                godot_error!("Load ts filed: {m}");
                return;
            }
        };
        self.ensure_initialized();
        let ctx = self.ctx().clone();
        let manager_ctx = self.context.clone();

        ctx.with(|ctx| {
            let deps = ProxyDeps {
                ctx: ctx.clone(),
                node: gd_node.clone().upcast(),
                manager_ctx,
            };
            let js_node_obj = create_godot_js_proxy(&deps);

            let res = js::Module::declare(ctx.clone(), file.path.clone(), file.source.clone())
                .and_then(|module| module.eval())
                .and_then(|(module, _)| {
                    let class_ctor: js::Function = module.get("default").unwrap();
                    let constructor = class_ctor
                        .as_constructor()
                        .ok_or(js::Error::InvalidExport)?;

                    let instance: js::Object = constructor.construct((js_node_obj,)).unwrap();

                    Ok(instance)
                });

            gdjs::util::with_handle_error(&ctx, res, |instance| {
                self.bounds.insert(
                    instance_id,
                    JsNodeBound {
                        js_object: js::Persistent::save(&ctx, instance.clone()),
                    },
                );
            });

            println!("Register queued done");
        });
        self.ready_queue.push(gd_node);
    }

    pub fn unregister_js_node(&mut self, id: InstanceId) {
        let Some(bound) = self.bounds.remove(&id) else {
            return;
        };
        self.free_queue.push(bound);
    }

    fn process_free(&mut self, ctx: &js::Ctx<'_>) {
        while let Some(bound) = self.free_queue.pop() {
            let instance = &bound.js_object.clone().restore(ctx);
            handle_error(ctx, instance);
            let Ok(instance) = instance else {
                return;
            };

            if instance.contains_key("onDestroy").unwrap_or(false) {
                gdjs::util::handle_error::<()>(ctx, &instance.call_method("onDestroy", ()));
            }
        }
    }

    pub fn enqueue_process(&mut self, id: InstanceId) {
        self.process_queue.push(id);
    }

    fn process_ready(&mut self, ctx: &js::Ctx<'_>) {
        if self.ready_queue.is_empty() {
            return;
        }

        while let Some(mut node) = self.ready_queue.pop().filter(|n| n.is_instance_valid())
            && let Some(bound) = self.bounds.get(&node.instance_id())
        {
            let Ok(instance) = &bound.js_object.clone().restore(ctx) else {
                return;
            };

            if instance.contains_key("onReady").unwrap_or(false) {
                gdjs::util::handle_error::<()>(ctx, &instance.call_method("onReady", ()));
            }

            if instance.contains_key("onProcess").unwrap_or(false) {
                node.set_process(true);
            } else {
                node.set_process(false);
            }
            println!("Register {} queued done", node.get_name());
        }
    }

    fn process_signals(&mut self, ctx: &js::Ctx<'_>) {
        let context_guard = self.context.borrow();
        if context_guard.queue_is_empty() {
            return;
        }
        drop(context_guard);

        self.context.process_queue(ctx);
    }

    fn update_debug(&mut self, dt: f64) {
        if !self.debug_enabled {
            return;
        }
        self.last_debug_update += dt;
        if self.last_debug_update < 0.5 {
            return;
        }
        self.last_debug_update = 0.0;
        if let Some(ref mut label) = self.debug_label {
            let uptime = self.start_time.elapsed().as_secs_f64();
            let fps = 1.0 / dt.max(0.001);

            let mut lines = String::new();

            if let Some(ref rt) = self.js_runtime {
                let mem = rt.memory_usage();
                lines.push_str(&format!(
                    "Mem:  {:.1} MB / {:.1} MB (lim: {:.1} MB)\n",
                    mem.memory_used_size as f64 / 1_048_576.0,
                    mem.malloc_size as f64 / 1_048_576.0,
                    mem.malloc_limit as f64 / 1_048_576.0,
                ));
                lines.push_str(&format!(
                    "GC:   obj={} str={} atm={} prop={} func={}\n",
                    mem.obj_count, mem.str_count, mem.atom_count, mem.prop_count, mem.js_func_count,
                ));
            }

            lines.push_str(&format!("Time: {:.0}s\n", uptime));
            lines.push_str(&format!("FPS:  {:.0}\n", fps));
            lines.push_str(&format!(
                "JS:   {} nodes (q:{})\n",
                self.bounds.len(),
                self.process_queue.len()
            ));

            let sig_count = self.context.borrow().queue_size();
            lines.push_str(&format!("Sig:  {} pending", sig_count));

            label.set_text(&lines);
        }
    }

    fn process_timers(&self, ctx: &js::Ctx<'_>, dt: f64) -> js::Result<()> {
        let mut timers = self.timers.borrow_mut();
        timers.tick(Duration::from_secs_f64(dt));
        if let Some(timer) = timers.pop_one() {
            drop(timers);
            let cb = timer.cb.restore(ctx)?;
            let _: () = cb.call(())?;
        }
        Ok(())
    }
}

#[godot_api]
impl INode for JsRuntimeManager {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            timers: JsTimers::new(),
            deadline: Default::default(),
            execution_limit: Default::default(),
            js_runtime: None,
            js_context: None,
            bounds: Default::default(),
            context: Default::default(),
            ready_queue: Default::default(),
            free_queue: Default::default(),
            process_queue: Vec::with_capacity(100),
            debug_enabled: false,
            start_time: Instant::now(),
            debug_label: None,
            last_debug_update: 0.5,
        }
    }

    fn ready(&mut self) {
        if !self.debug_enabled {
            return;
        }

        let mut canvas = CanvasLayer::new_alloc();
        canvas.set_layer(128);
        self.base_mut().add_child(&canvas);

        let mut label = Label::new_alloc();
        label.set_text("JS Runtime Debug");
        label.set_scale(Vector2::new(0.75, 0.75));
        label.set_anchors_preset(LayoutPreset::TOP_LEFT);
        label.set_position(Vector2::new(8.0, 8.0));
        canvas.add_child(&label);
        self.debug_label = Some(label);
        self.start_time = Instant::now();
    }

    fn process(&mut self, dt: f64) {
        let ctx = self.ctx().clone();

        let _guard = DeadlineGuard::new(self.deadline.clone(), self.execution_limit as u64);
        ctx.with(|ctx| {
            self.process_free(&ctx);
            self.process_ready(&ctx);
            for i in self.process_queue.iter() {
                if let Some(bound) = self.bounds.get(i) {
                    gdjs::util::handle_error::<()>(
                        &ctx,
                        &bound
                            .js_object
                            .clone()
                            .restore(&ctx)
                            .and_then(|instance| instance.call_method("onProcess", (dt,))),
                    );
                    while ctx.execute_pending_job() {} // run microtasks
                }
            }
            self.process_signals(&ctx);
            gdjs::util::handle_error(&ctx, &self.process_timers(&ctx, dt));
            ctx.run_gc();
        });
        self.process_queue.clear();

        self.update_debug(dt);
    }

    fn exit_tree(&mut self) {
        let mut timers = self.timers.borrow_mut();
        timers.clear();
        drop(timers);
        self.process_queue.clear();
        self.ready_queue.clear();
        self.free_queue.clear();
        self.bounds.clear();
        self.context.borrow_mut().clear();
    }
}
