use crate::js_core::godot::converters::{godot_variant_to_js, js_to_godot_variant};
use crate::js_core::proxy_vec::create_vector2_proxy;
use crate::js_core::utils::{extract_trace, gd_alive_handle};
use crate::js_core::vectors::JsVector2;
use crate::{
    manager::{self, JsRuntimeManager},
    prelude::*,
};

use rquickjs::Class;
use rquickjs::{IntoJs, Proxy, class::Trace, methods, proxy::ProxyHandler};

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
            manager.bind_mut().process_enqueue(instance_id);
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

pub fn create_godot_js_proxy<'js>(
    ctx: &Ctx<'js>,
    godot_node: Gd<Node>,
) -> rquickjs::Result<Value<'js>> {
    let target = JsNodeProxy {
        godot_node: godot_node.clone(),
    };

    let target_js = target.into_js(ctx)?;

    let handler = Object::new(ctx.clone())?;

    let node = godot_node.clone();
    let get_trap = Function::new(
        ctx.clone(),
        move |ctx: Ctx<'js>, target_obj: Object<'js>, prop: String| -> JsResult<Value<'js>> {
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

            match prop.as_str() {
                "name" => {
                    let name_str = node.get_name().to_string();
                    return Ok(rquickjs::String::from_str(ctx.clone(), name_str.as_str())
                        .map(|js_s| js_s.into_value())
                        .unwrap_or_else(|_| Value::new_undefined(ctx.clone())));
                }
                "id" => {
                    return Ok(Value::new_number(
                        ctx.clone(),
                        node.instance_id().to_i64() as f64,
                    ));
                }
                "parent" => {
                    if let Some(parent_node) = node.get_parent()
                        && let Ok(parent_proxy) = create_godot_js_proxy(&ctx, parent_node)
                    {
                        return Ok(parent_proxy);
                    }
                    return Ok(Value::new_null(ctx.clone()));
                }
                _ => {}
            }

            let string_name = StringName::from(&prop);

            if node.get(&string_name).try_to::<Vector2>().is_ok() {
                let proxy = create_vector2_proxy(&ctx, node.clone(), string_name.clone())?;
                target_obj.set(&cache_prop_name, proxy.clone())?;
                return Ok(proxy);
            }

            if node.has_method(&string_name) {
                let node_to_call = node.clone();

                let string_name = string_name.clone();
                if let Ok(call_closure) = Function::new(
                    ctx.clone(),
                    move |ctx: Ctx<'js>, args: rquickjs::function::Rest<Value<'js>>| {
                        let godot_args: Vec<Variant> = args
                            .0
                            .into_iter()
                            .map(|v| js_to_godot_variant(&ctx, v))
                            .collect();

                        let mut local_node = node_to_call.clone();

                        let res = local_node.call(&string_name, &godot_args);
                        godot_variant_to_js(&ctx, res)
                    },
                ) {
                    return Ok(call_closure.into_value());
                }
            }

            let godot_variant = node.get(&string_name);
            godot_variant_to_js(&ctx, godot_variant)
        },
    )?;

    let node = godot_node;
    let set_trap = Function::new(
        ctx.clone(),
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
    )?;

    handler.set("get", get_trap)?;
    handler.set("set", set_trap)?;

    let proxy = Proxy::new(ctx.clone(), target_js, ProxyHandler::from_object(handler)?)?;
    Ok(proxy.into_value())
}
