use std::collections::HashMap;

use rquickjs::Persistent;

use crate::node::create_godot_js_proxy;
pub use crate::prelude::*;
use crate::proxy_deps::ProxyDeps;
pub struct JsNodeBound {
    pub js_object: Persistent<Object<'static>>,
}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct JsRuntimeManager {
    base: Base<Node>,
    js_runtime: Option<Runtime>,
    js_context: Option<Context>,
    bounds: HashMap<InstanceId, JsNodeBound>,
    process_queue: Vec<InstanceId>,
    context: ManagerCtxRef,
}

#[godot_api]
impl JsRuntimeManager {
    fn ensure_initialized(&mut self) {
        if self.js_context.is_some() {
            return;
        }

        godot_print!("--- JS Engine initialization.. ---");

        let runtime = Runtime::new().expect("JS Runtime create failure");
        let context = Context::full(&runtime).expect("JS Context failure");

        context.with(|ctx| {
            let res = (|| -> rquickjs::Result<()> {
                let globals = ctx.globals();
                let console_obj = gdjs::console::create(&ctx);
                globals.set("console", console_obj)?;
                Ok(())
            })();
            gdjs::util::handle_error(&ctx, &res);
        });

        self.js_runtime = Some(runtime);
        self.js_context = Some(context);
    }

    fn ctx(&self) -> &Context {
        self.js_context.as_ref().expect("Context must be inited")
    }

    pub fn register_js_node(&mut self, mut gd_node: Gd<Node>, script_path: String) {
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
                node: gd_node.clone(),
                manager_ctx,
            };
            let js_node_obj = create_godot_js_proxy(&deps);

            let res =
                rquickjs::Module::declare(ctx.clone(), file.path.clone(), file.source.clone())
                    .and_then(|module| module.eval())
                    .and_then(|(module, _)| {
                        let class_ctor: rquickjs::Function = module.get("default").unwrap();
                        let constructor = class_ctor
                            .as_constructor()
                            .ok_or(rquickjs::Error::InvalidExport)?;

                        let instance: rquickjs::Object =
                            constructor.construct((js_node_obj,)).unwrap();

                        Ok(instance)
                    });

            gdjs::util::with_handle_error(&ctx, res, |instance| {
                if instance.contains_key("onReady").unwrap_or(false) {
                    gdjs::util::handle_error::<()>(&ctx, &instance.call_method("onReady", ()));
                }

                if instance.contains_key("onProcess").unwrap_or(false) {
                    self.bounds.insert(
                        instance_id,
                        JsNodeBound {
                            js_object: Persistent::save(&ctx, instance.clone()),
                        },
                    );
                    gd_node.set_process(true);
                } else {
                    gd_node.set_process(false);
                }
            });

            println!("Register done");
        });
    }

    pub fn unregister_js_node(&mut self, id: InstanceId) {
        self.bounds.remove(&id);
    }

    pub fn enqueue_process(&mut self, id: InstanceId) {
        self.process_queue.push(id);
    }

    fn process_signals(&mut self) {
        let context_guard = self.context.borrow();
        if context_guard.queue_is_empty() {
            return;
        }
        drop(context_guard);

        let ctx = self.ctx();
        ctx.with(|ctx| {
            self.context.borrow_mut().process_queue(&ctx);
        });
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
            process_queue: Vec::with_capacity(100),
        }
    }

    fn process(&mut self, dt: f64) {
        for i in self.process_queue.iter() {
            if let Some(bound) = self.bounds.get(i) {
                self.ctx().with(|ctx| {
                    gdjs::util::handle_error::<()>(
                        &ctx,
                        &bound
                            .js_object
                            .clone()
                            .restore(&ctx)
                            .and_then(|instance| instance.call_method("onProcess", (dt,))),
                    );
                    while ctx.execute_pending_job() {} // run microtasks
                });
            }
        }
        self.process_queue.clear();
        self.process_signals();
    }

    fn exit_tree(&mut self) {
        self.context.borrow_mut().clear();
        self.bounds.clear();
        self.process_queue.clear();
    }
}
