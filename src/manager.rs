use std::collections::HashMap;

use rquickjs::Persistent;

pub use crate::prelude::*;
use crate::{
    js_core, js_utility,
    node::{JsNodeProxy, create_godot_js_proxy},
};

pub struct JsNodeBound {
    pub js_object: Persistent<Object<'static>>,
}

#[derive(GodotClass)]
#[class(base=Node)]
pub struct JsRuntimeManager {
    base: Base<Node>,
    runtime: Option<Runtime>,
    context: Option<Context>,
    bounds: HashMap<InstanceId, JsNodeBound>,
    process_queue: Vec<InstanceId>,
}

#[godot_api]
impl JsRuntimeManager {
    fn ensure_initialized(&mut self) {
        if self.context.is_some() {
            return;
        }

        godot_print!("--- JS Engine initialization.. ---");

        let runtime = Runtime::new().expect("JS Runtime create failure");
        let context = Context::full(&runtime).expect("JS Context failure");

        context.with(|ctx| {
            let res = (|| -> rquickjs::Result<()> {
                let globals = ctx.globals();
                let console_obj = js_core::console::create(&ctx);
                globals.set("console", console_obj)?;
                Ok(())
            })();
            js_utility::handle_error(&ctx, &res);
        });

        self.runtime = Some(runtime);
        self.context = Some(context);
    }

    fn ctx(&self) -> &Context {
        self.context.as_ref().expect("Context must be inited")
    }

    pub fn register_js_node(&mut self, mut gd_node: Gd<Node>, script_path: String) {
        let instance_id = gd_node.instance_id();

        let file = match crate::js_utility::load_js(&script_path) {
            Some(f) => f,
            None => return,
        };
        self.ensure_initialized();
        let ctx = self.ctx().clone();

        ctx.with(|ctx| {
            let js_node_obj = create_godot_js_proxy(&ctx, gd_node.clone());
            // js_node_obj.set("id", instance_id.to_i64()).unwrap();

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

            js_utility::with_handle_error(&ctx, res, |instance| {
                if instance.contains_key("onReady").unwrap_or(false) {
                    js_utility::handle_error::<()>(&ctx, &instance.call_method("onReady", ()));
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

    pub fn process_enqueue(&mut self, id: InstanceId) {
        self.process_queue.push(id);
    }
}

#[godot_api]
impl INode for JsRuntimeManager {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            runtime: None,
            context: None,
            bounds: Default::default(),
            process_queue: Vec::with_capacity(100),
        }
    }

    fn process(&mut self, dt: f64) {
        for i in self.process_queue.iter() {
            if let Some(bound) = self.bounds.get(i) {
                self.ctx().with(|ctx| {
                    js_utility::handle_error::<()>(
                        &ctx,
                        &bound
                            .js_object
                            .clone()
                            .restore(&ctx)
                            .and_then(|instance| instance.call_method("onProcess", (dt,))),
                    );
                });
            }
        }
        self.process_queue.clear();
    }
}
