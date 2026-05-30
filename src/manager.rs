use std::collections::HashMap;
use std::time::Instant;

use godot::classes::control::LayoutPreset;
use godot::classes::{CanvasLayer, Label};
use rquickjs::loader::ModuleLoader;

use crate::bifrost_js_module::BifrostModule;
use crate::node::create_godot_js_proxy;
pub use crate::prelude::*;
use crate::proxy_deps::ProxyDeps;
pub struct JsNodeBound {
    pub js_object: js::Persistent<js::Object<'static>>,
}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct JsRuntimeManager {
    base: Base<Node>,
    js_runtime: Option<js::Runtime>,
    js_context: Option<js::Context>,
    bounds: HashMap<InstanceId, JsNodeBound>,
    process_queue: Vec<InstanceId>,
    register_queue: Vec<Gd<Node>>,
    context: ManagerCtxRef,

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

        let context = js::Context::full(&runtime).expect("JS Context failure");

        context.with(|ctx| {
            let res = (|| -> js::Result<()> {
                let globals = ctx.globals();
                let console_obj = gdjs::console::create(&ctx);
                globals.set("console", console_obj)?;
                js::Module::evaluate_def::<BifrostModule, _>(ctx.clone(), "bifrostjs")?;
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

    pub fn register_js_node(&mut self, gd_node: Gd<Node>, script_path: String) {
        let instance_id = gd_node.instance_id();

        let file = match gdjs::util::load_js(&script_path) {
            Some(f) => f,
            None => return,
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

                // if instance.contains_key("onReady").unwrap_or(false) {
                //     gdjs::util::handle_error::<()>(&ctx, &instance.call_method("onReady", ()));
                // }
                //
                // if instance.contains_key("onProcess").unwrap_or(false) {
                //     gd_node.set_process(true);
                // } else {
                //     gd_node.set_process(false);
                // }
            });

            println!("Register queued done");
        });
        self.register_queue.push(gd_node);
    }

    pub fn unregister_js_node(&mut self, id: InstanceId) {
        self.bounds.remove(&id);
    }

    pub fn enqueue_process(&mut self, id: InstanceId) {
        self.process_queue.push(id);
    }

    fn process_registrations(&mut self, ctx: &js::Ctx<'_>) {
        if self.register_queue.is_empty() {
            return;
        }

        while let Some(mut node) = self.register_queue.pop().filter(|n| n.is_instance_valid())
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
}

#[godot_api]
impl INode for JsRuntimeManager {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            js_runtime: None,
            js_context: None,
            bounds: Default::default(),
            context: Default::default(),
            register_queue: Default::default(),
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
        ctx.with(|ctx| {
            self.process_registrations(&ctx);
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
            ctx.run_gc();
        });
        self.process_queue.clear();

        self.update_debug(dt);
    }

    fn exit_tree(&mut self) {
        self.context.borrow_mut().clear();
        self.bounds.clear();
        self.process_queue.clear();
        self.register_queue.clear();
    }
}
