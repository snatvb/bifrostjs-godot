use godot::prelude::*;
use js_core::js::{self, IntoJs};

use crate::util::gd_alive_handle;

pub fn create_rect2_proxy<'js>(
    ctx: &js::Ctx<'js>,
    gdobject: Gd<godot::prelude::Object>,
    prop_name: StringName,
) -> js::Result<js::Value<'js>> {
    let gdnode = match gdobject.try_cast::<Node>() {
        Ok(n) => n,
        Err(_) => {
            return Err(ctx.throw("Rect2 proxy: not a Node".into_js(ctx)?));
        }
    };
    let rect_target = js::Object::new(ctx.clone())?;
    let rect_handler = js::Object::new(ctx.clone())?;

    let node_get = gdnode.clone();
    let prop_get = prop_name.clone();
    let get = js::Function::new(
        ctx.clone(),
        move |ctx: js::Ctx<'js>,
              _target: js::Object<'js>,
              prop: String|
              -> js::Result<js::Value<'js>> {
            let alive = node_get.is_instance_valid();
            if prop == "is_alive" {
                return Ok(js::Value::new_bool(ctx.clone(), alive));
            }

            gd_alive_handle(&ctx, alive)?;

            let current_val = node_get.get(&prop_get);
            let r2 = match current_val.try_to::<Rect2>() {
                Ok(v) => v,
                Err(_) => {
                    return Err(ctx.throw("Rect2 proxy: cannot read Godot Rect2".into_js(&ctx)?));
                }
            };
            let res = match prop.as_str() {
                "x" => r2.position.x,
                "y" => r2.position.y,
                "width" => r2.size.x,
                "height" => r2.size.y,
                _ => 0.0,
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
            if gd_alive_handle(&ctx, alive).is_err() {
                return false;
            }
            let current_val = node_set.get(&prop_set);
            let Ok(mut r2) = current_val.try_to::<Rect2>() else {
                let _ = ctx.throw(
                    js::String::from_str(
                        ctx.clone(),
                        "Rect2 proxy: cannot read Godot Rect2 for mutation",
                    )
                    .unwrap()
                    .into_value(),
                );
                return false;
            };
            match prop.as_str() {
                "x" => r2.position.x = val,
                "y" => r2.position.y = val,
                "width" => r2.size.x = val,
                "height" => r2.size.y = val,
                _ => {
                    let _ = ctx.throw(
                        js::String::from_str(
                            ctx.clone(),
                            format!("Rect2 proxy: unknown property '{}'", prop).as_str(),
                        )
                        .unwrap()
                        .into_value(),
                    );
                    return false;
                }
            }
            let mut node_set = node_set.clone();
            node_set.set(&prop_set, &Variant::from(r2));
            true
        },
    )?;

    rect_handler.set("get", get)?;
    rect_handler.set("set", set)?;

    let proxy = js::Proxy::new(
        ctx.clone(),
        rect_target,
        js::proxy::ProxyHandler::from_object(rect_handler)?,
    )?;
    Ok(proxy.into_value())
}
