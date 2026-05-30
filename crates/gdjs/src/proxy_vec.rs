use godot::prelude::*;
use js_core::js;

use crate::util::gd_alive_handle;

pub fn create_vector2_proxy<'js>(
    ctx: &js::Ctx<'js>,
    gdobject: Gd<godot::prelude::Object>,
    prop_name: StringName,
) -> js::Result<js::Value<'js>> {
    let gdnode = match gdobject.try_cast::<Node>() {
        Ok(n) => n,
        Err(_) => return Ok(js::Value::new_undefined(ctx.clone())),
    };
    let vec_target = js::Object::new(ctx.clone())?;
    let vec_handler = js::Object::new(ctx.clone())?;

    let node_get = gdnode.clone();
    let prop_get = prop_name.clone();
    let get = js::Function::new(
        ctx.clone(),
        move |ctx: js::Ctx<'js>, _target: js::Object<'js>, prop: String| -> js::Result<js::Value<'js>> {
            let alive = node_get.is_instance_valid();
            if prop == "is_alive" {
                return Ok(js::Value::new_bool(ctx.clone(), alive));
            }

            gd_alive_handle(&ctx, alive)?;

            let current_val = node_get.get(&prop_get);
            let res = if let Ok(v2) = current_val.try_to::<Vector2>() {
                match prop.as_str() {
                    "x" => v2.x,
                    "y" => v2.y,
                    _ => 0.0,
                }
            } else {
                0.0f32
            };
            Ok(js::Value::new_number(ctx, res as f64))
        },
    )?;

    let node_set = gdnode;
    let prop_set = prop_name;
    let set = js::Function::new(
        ctx.clone(),
        move |ctx: js::Ctx<'js>, _target: js::Object<'js>, prop: String, val: f32| -> bool {
            let alive = node_set.is_instance_valid();
            if let Err(err) = gd_alive_handle(&ctx, alive) {
                godot_error!("{}", err);
                return false;
            }
            let current_val = node_set.get(&prop_set);
            if let Ok(mut v2) = current_val.try_to::<Vector2>() {
                match prop.as_str() {
                    "x" => v2.x = val,
                    "y" => v2.y = val,
                    _ => return false,
                }
                let mut node_set = node_set.clone();
                node_set.set(&prop_set, &Variant::from(v2));
                return true;
            }
            false
        },
    )?;

    vec_handler.set("get", get)?;
    vec_handler.set("set", set)?;

    let proxy = js::Proxy::new(
        ctx.clone(),
        vec_target,
        js::proxy::ProxyHandler::from_object(vec_handler)?,
    )?;
    Ok(proxy.into_value())
}
