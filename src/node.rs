use crate::{manager::JsRuntimeManager, prelude::*};
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

pub fn create_godot_js_proxy<'js>(
    ctx: &Ctx<'js>,
    godot_node: Gd<Node>,
) -> rquickjs::Result<Value<'js>> {
    let target = JsNodeProxy {
        godot_node: godot_node.clone(),
    };
    let target_js = target.into_js(ctx)?;

    let handler = Object::new(ctx.clone())?;
    handler.set("get", make_get_trap(ctx, godot_node.clone())?)?;
    handler.set("set", make_set_trap(ctx, godot_node)?)?;

    let proxy = Proxy::new(ctx.clone(), target_js, ProxyHandler::from_object(handler)?)?;
    Ok(proxy.into_value())
}

fn make_get_trap<'js>(ctx: &Ctx<'js>, node: Gd<Node>) -> rquickjs::Result<Function<'js>> {
    Function::new(
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

            if let Some(val) = get_special_property(&ctx, &node, &prop)? {
                return Ok(val);
            }

            let string_name = StringName::from(&prop);

            if node.get(&string_name).try_to::<Vector2>().is_ok() {
                let proxy = create_vector2_proxy(&ctx, node.clone(), string_name.clone())?;
                target_obj.set(&cache_prop_name, proxy.clone())?;
                return Ok(proxy);
            }

            if node.has_method(&string_name) {
                return handle_method_call(&ctx, &target_obj, &node, &prop, string_name);
            }

            let godot_variant = node.get(&string_name);
            godot_variant_to_js(&ctx, godot_variant)
        },
    )
}

fn make_cache_fn_name(prop: &str) -> String {
    format!("_cache_fn_{prop}")
}

fn handle_method_call<'js>(
    ctx: &Ctx<'js>,
    target_obj: &Object<'js>,
    node: &Gd<Node>,
    prop: &str,
    string_name: StringName,
) -> JsResult<Value<'js>> {
    let cache_fn_name = make_cache_fn_name(prop);
    let cached_fn: Value<'js> = target_obj.get(&cache_fn_name)?;
    if !cached_fn.is_undefined() && !cached_fn.is_null() {
        return Ok(cached_fn);
    }

    let method_fn = get_special_method(ctx, node, prop)?
        .map(Ok)
        .unwrap_or_else(|| make_godot_method_fn(ctx, node.clone(), string_name))?;
    target_obj.set(&cache_fn_name, method_fn.clone())?;
    Ok(method_fn.into_value())
}

fn make_godot_method_fn<'js>(
    ctx: &Ctx<'js>,
    node: Gd<Node>,
    method_name: StringName,
) -> rquickjs::Result<Function<'js>> {
    Function::new(
        ctx.clone(),
        move |ctx: Ctx<'js>, args: Rest<Value<'js>>| -> rquickjs::Result<Value<'js>> {
            check_alive_handle(&ctx, &node)?;

            let result_variant = node.clone().call(&method_name, &js_to_gd_args(&ctx, args));
            if result_variant.get_type() == VariantType::OBJECT
                && let Ok(Ok(obj)) = result_variant
                    .try_to::<Gd<godot::prelude::Object>>()
                    .map(|o| o.try_cast::<Node>())
            {
                return create_godot_js_proxy(&ctx, obj);
            }
            godot_variant_to_js(&ctx, result_variant)
        },
    )
}

fn get_special_method<'js>(
    ctx: &Ctx<'js>,
    node: &Gd<Node>,
    prop: &str,
) -> rquickjs::Result<Option<Function<'js>>> {
    Ok(Some(match prop {
        "is_class" => make_is_class(ctx, node)?,
        _ => return Ok(None),
    }))
}

fn get_special_property<'js>(
    ctx: &Ctx<'js>,
    node: &Gd<Node>,
    prop: &str,
) -> rquickjs::Result<Option<Value<'js>>> {
    Ok(Some(match prop {
        "name" => {
            let name_str = node.get_name().to_string();
            rquickjs::String::from_str(ctx.clone(), name_str.as_str())
                .map(|js_s| js_s.into_value())?
        }
        "class_type" => {
            let class_name = node.get_class().to_string();
            let js_str = rquickjs::String::from_str(ctx.clone(), class_name.as_str())?;
            js_str.into_value()
        }
        "id" => Value::new_number(ctx.clone(), node.instance_id().to_i64() as f64),
        "parent" => match node.get_parent() {
            Some(parent) => return create_godot_js_proxy(ctx, parent).map(Some),
            None => Value::new_null(ctx.clone()),
        },
        _ => return Ok(None),
    }))
}

fn make_is_class<'js>(ctx: &Ctx<'js>, node: &Gd<Node>) -> rquickjs::Result<Function<'js>> {
    let node = node.clone();
    let is_class_fn = Function::new(
        ctx.clone(),
        move |ctx: Ctx<'js>, class_name: String| -> rquickjs::Result<bool> {
            if !node.is_instance_valid() {
                let js_val = "Cannot check class on a deleted Godot Node!".into_js(&ctx)?;
                return Err(ctx.throw(js_val));
            }

            let is_inherited = node.is_class(&class_name);
            Ok(is_inherited)
        },
    )?;

    Ok(is_class_fn)
}

fn make_set_trap<'js>(ctx: &Ctx<'js>, node: Gd<Node>) -> rquickjs::Result<Function<'js>> {
    Function::new(
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
    )
}
