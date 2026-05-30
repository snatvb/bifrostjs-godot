use crate::manager::JsRuntimeManager;
use crate::prelude::*;
use crate::proxy_deps::ProxyDeps;
use gdjs::converters::{godot_variant_to_js, js_to_gd_args, js_to_godot_variant};
use gdjs::proxy_vec::create_vector2_proxy;
use gdjs::util::{check_alive_handle, gd_alive_handle};
use js_core::utils::extract_trace;

use rquickjs::prelude::Rest;
use rquickjs::{IntoJs, Proxy, class::Trace, proxy::ProxyHandler};

#[derive(GodotClass)]
#[class(base=Node)]
pub struct JsNode {
    base: Base<Node>,

    #[export]
    script_path: GString,

    manager: Option<Gd<JsRuntimeManager>>,
}

#[godot_api]
impl INode for JsNode {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            script_path: GString::from(""),
            manager: None,
        }
    }

    fn process(&mut self, _: f64) {
        let instance_id = self.base().instance_id();
        if let Some(manager) = self.manager.as_mut() {
            manager.bind_mut().enqueue_process(instance_id);
        }
    }

    fn ready(&mut self) {
        if self.script_path.is_empty() {
            godot_warn!("JsNode [{}] has no filled path!", self.base().get_name());
            return;
        }

        let mut current_parent = self.base().get_parent();
        let mut found_manager: Option<Gd<JsRuntimeManager>> = None;
        while let Some(parent) = current_parent {
            match parent.try_cast::<JsRuntimeManager>() {
                Ok(manager_gd) => {
                    found_manager = Some(manager_gd);
                    break;
                }
                Err(returned_parent) => {
                    current_parent = returned_parent.get_parent();
                }
            }
        }

        if let Some(mut manager) = found_manager {
            self.manager = Some(manager.clone());
            println!("Register!");
            manager
                .bind_mut()
                .register_js_node(self.base().to_godot().clone(), self.script_path.to_string());
        }
    }
    fn exit_tree(&mut self) {
        let id = self.base().instance_id();
        if let Some(manager) = self.manager.as_mut() {
            manager.bind_mut().unregister_js_node(id);
        }
    }
}

#[derive(rquickjs::JsLifetime)]
pub struct JsNodeProxy {
    #[allow(dead_code)]
    pub godot_node: Gd<Node>,
}

impl<'js> Trace<'js> for JsNodeProxy {
    fn trace<'a>(&self, _tracer: rquickjs::class::Tracer<'a, 'js>) {}
}

impl<'js> IntoJs<'js> for JsNodeProxy {
    fn into_js(self, ctx: &Ctx<'js>) -> rquickjs::Result<Value<'js>> {
        let obj = Object::new(ctx.clone())?;
        Ok(obj.into_value())
    }
}

pub fn create_godot_js_proxy<'js>(deps: &ProxyDeps<'js>) -> rquickjs::Result<Value<'js>> {
    let target = JsNodeProxy {
        godot_node: deps.node.clone(),
    };
    let target_js = target.into_js(&deps.ctx)?;

    let handler = Object::new(deps.ctx.clone())?;
    handler.set("get", make_get_trap(deps)?)?;
    handler.set("set", make_set_trap(deps)?)?;

    let proxy = Proxy::new(
        deps.ctx.clone(),
        target_js,
        ProxyHandler::from_object(handler)?,
    )?;
    Ok(proxy.into_value())
}

fn make_get_trap<'js>(deps: &ProxyDeps<'js>) -> rquickjs::Result<Function<'js>> {
    let node = deps.node.clone();
    let man_ctx = deps.manager_ctx.clone();
    Function::new(
        deps.ctx.clone(),
        move |ctx: Ctx<'js>, target_obj: Object<'js>, prop: String| -> JsResult<Value<'js>> {
            let deps = ProxyDeps {
                ctx: ctx.clone(),
                node: node.clone(),
                manager_ctx: man_ctx.clone(),
            };

            if target_obj.contains_key(&prop).unwrap_or(false) {
                return Ok(target_obj
                    .get(&prop)
                    .unwrap_or_else(|_| Value::new_undefined(ctx.clone())));
            }

            let alive = node.is_instance_valid();
            if prop == "is_alive" {
                return Ok(Value::new_bool(ctx.clone(), alive));
            }

            gd_alive_handle(&ctx, alive)?;

            let cache_prop_name = format!("_cache_{}", prop);
            if target_obj.contains_key(&cache_prop_name).unwrap_or(false)
                && let Ok(cache) = target_obj.get(&cache_prop_name)
            {
                return cache;
            }

            if let Some(val) = get_special_property(&deps, &prop)? {
                return Ok(val);
            }

            let string_name = StringName::from(&prop);

            if node.get(&string_name).try_to::<Vector2>().is_ok() {
                let proxy = create_vector2_proxy(&ctx, node.clone(), string_name.clone())?;
                target_obj.set(&cache_prop_name, proxy.clone())?;
                return Ok(proxy);
            }

            if node.has_method(&string_name) {
                return handle_method_call(&deps, &target_obj, &prop, string_name);
            }

            let godot_variant = node.get(&string_name);
            godot_variant_to_js(&ctx, godot_variant)
        },
    )
}

#[inline(always)]
fn make_cache_fn_name(prop: &str) -> String {
    format!("_cache_fn_{prop}")
}

fn handle_method_call<'js>(
    deps: &ProxyDeps<'js>,
    target_obj: &Object<'js>,
    prop: &str,
    string_name: StringName,
) -> JsResult<Value<'js>> {
    let cache_fn_name = make_cache_fn_name(prop);
    let cached_fn: Value<'js> = target_obj.get(&cache_fn_name)?;
    if !cached_fn.is_undefined() && !cached_fn.is_null() {
        return Ok(cached_fn);
    }

    let method_value = match get_special_method(deps, prop)? {
        Some(v) => v,
        None => make_godot_method_fn(deps, string_name)?.into_value(),
    };
    target_obj.set(&cache_fn_name, method_value.clone())?;
    Ok(method_value)
}

fn make_godot_method_fn<'js>(
    deps: &ProxyDeps<'js>,
    method_name: StringName,
) -> rquickjs::Result<Function<'js>> {
    let node = deps.node.clone();
    let man_ctx = deps.manager_ctx.clone();
    Function::new(
        deps.ctx.clone(),
        move |ctx: Ctx<'js>, args: Rest<Value<'js>>| -> rquickjs::Result<Value<'js>> {
            check_alive_handle(&ctx, &node)?;

            let result_variant = node.clone().call(&method_name, &js_to_gd_args(&ctx, args));
            if result_variant.get_type() == VariantType::OBJECT
                && let Ok(Ok(obj)) = result_variant
                    .try_to::<Gd<godot::prelude::Object>>()
                    .map(|o| o.try_cast::<Node>())
            {
                let child_deps = ProxyDeps {
                    ctx: ctx.clone(),
                    node: obj,
                    manager_ctx: man_ctx.clone(),
                };
                return create_godot_js_proxy(&child_deps);
            }
            godot_variant_to_js(&ctx, result_variant)
        },
    )
}

fn get_special_method<'js>(
    deps: &ProxyDeps<'js>,
    prop: &str,
) -> rquickjs::Result<Option<Value<'js>>> {
    Ok(Some(match prop {
        "is_class" => make_is_class(&deps.ctx, &deps.node)?.into_value(),
        "connect" => make_connect(deps)?.into_value(),
        _ => return Ok(None),
    }))
}

// fn make_disconnect<'js>(deps: &ProxyDeps<'js>) -> JsResult<Function<'js>> {
//     let context_weak = deps.manager_ctx.downgrade();
//     let node = deps.node.clone();
//     let connect_fn = Function::new(
//         deps.ctx.clone(),
//         move |ctx: Ctx<'js>,
//               signal_name: String,
//               callback: Function<'js>|
//               -> rquickjs::Result<()> {
//             check_alive_handle(&ctx, &node)?;
//             if let Some(context) = context_weak.upgrade() {
//                 let id = context.borrow_mut().save_callback(&ctx, callback);
//                 let bridge = JsSignalBridge::create(id, context_weak.clone());
//                 let callable = Callable::from_object_method(&bridge, "on_signal_fired");
//                 let mut node = node.clone();
//                 node.connect(&StringName::from(&signal_name), &callable);
//             }
//             Ok(())
//         },
//     )?;
//     Ok(connect_fn)
// }

fn make_connect<'js>(deps: &ProxyDeps<'js>) -> JsResult<Function<'js>> {
    let context_weak = deps.manager_ctx.downgrade();
    let node = deps.node.clone();
    let connect_fn = Function::new(
        deps.ctx.clone(),
        move |ctx: Ctx<'js>,
              signal_name: String,
              callback: Function<'js>|
              -> rquickjs::Result<()> {
            check_alive_handle(&ctx, &node)?;
            if let Some(context) = context_weak.upgrade() {
                let id = context.borrow_mut().save_callback(&ctx, callback);
                let ctx_weak = context_weak.clone();
                let callable = Callable::from_linked_fn(
                    "js_signal_handler",
                    &node,
                    move |_args: &[&godot::prelude::Variant]| {
                        if let Some(context) = ctx_weak.upgrade() {
                            context.borrow_mut().enqueue(id);
                        }

                        godot::prelude::Variant::nil()
                    },
                );
                let mut node = node.clone();
                println!("Connect on {signal_name}");
                node.connect(&StringName::from(&signal_name), &callable);
            }
            Ok(())
        },
    )?;
    Ok(connect_fn)
}

fn get_special_property<'js>(
    deps: &ProxyDeps<'js>,
    prop: &str,
) -> rquickjs::Result<Option<Value<'js>>> {
    Ok(Some(match prop {
        "name" => {
            let name_str = deps.node.get_name().to_string();
            rquickjs::String::from_str(deps.ctx.clone(), name_str.as_str())
                .map(|js_s| js_s.into_value())?
        }
        "class_type" => {
            let class_name = deps.node.get_class().to_string();
            let js_str = rquickjs::String::from_str(deps.ctx.clone(), class_name.as_str())?;
            js_str.into_value()
        }
        "id" => Value::new_number(deps.ctx.clone(), deps.node.instance_id().to_i64() as f64),
        "parent" => match deps.node.get_parent() {
            Some(parent) => return create_godot_js_proxy(&deps.with_node(parent)).map(Some),
            None => Value::new_null(deps.ctx.clone()),
        },
        _ => return Ok(None),
    }))
}

fn make_is_class<'js>(ctx: &Ctx<'js>, node: &Gd<Node>) -> rquickjs::Result<Function<'js>> {
    let node = node.clone();
    let is_class_fn = Function::new(
        ctx.clone(),
        move |ctx: Ctx<'js>, class_name: String| -> rquickjs::Result<bool> {
            check_alive_handle(&ctx, &node)?;

            let is_inherited = node.is_class(&class_name);
            Ok(is_inherited)
        },
    )?;

    Ok(is_class_fn)
}

fn make_set_trap<'js>(deps: &ProxyDeps<'js>) -> rquickjs::Result<Function<'js>> {
    let node = deps.node.clone();
    Function::new(
        deps.ctx.clone(),
        move |ctx: Ctx<'js>, _target_obj: Object<'js>, prop: String, value: Value<'js>| -> bool {
            let alive = node.is_instance_valid();
            if let Err(err) = gd_alive_handle(&ctx, alive) {
                godot_error!("{}", err);
                return false;
            }
            let string_name = StringName::from(&prop);

            if node.get(&string_name).try_to::<Vector2>().is_ok() {
                godot_error!("Mutate of {} is forbidden\n{}", prop, extract_trace(&ctx));
                return false;
            }
            let godot_variant = js_to_godot_variant(&ctx, value);

            let mut godot_node_cloned_set = node.clone();
            godot_node_cloned_set.set(&string_name, &godot_variant);
            true
        },
    )
}
