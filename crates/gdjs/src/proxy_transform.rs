use godot::prelude::*;
use js_core::js::{self, IntoJs};

use crate::util::{col_cache_key, gd_alive_handle, with_cache};

pub fn create_transform2d_proxy<'js>(
    ctx: &js::Ctx<'js>,
    gdobject: Gd<godot::prelude::Object>,
    prop_name: StringName,
) -> js::Result<js::Value<'js>> {
    let gdnode = match gdobject.try_cast::<Node>() {
        Ok(n) => n,
        Err(_) => {
            return Err(ctx.throw("Transform2D proxy: not a Node".into_js(ctx)?));
        }
    };
    let t2d_target = js::Object::new(ctx.clone())?;
    let t2d_handler = js::Object::new(ctx.clone())?;

    let node_get = gdnode.clone();
    let prop_get = prop_name.clone();
    let get = js::Function::new(
        ctx.clone(),
        move |ctx: js::Ctx<'js>,
              target: js::Object<'js>,
              prop: String|
              -> js::Result<js::Value<'js>> {
            let alive = node_get.is_instance_valid();
            if prop == "is_alive" {
                return Ok(js::Value::new_bool(ctx.clone(), alive));
            }
            gd_alive_handle(&ctx, alive)?;

            let cache_key = col_cache_key(&prop);
            if target.contains_key(&cache_key).unwrap_or(false) {
                if let Ok(cached) = target.get::<_, js::Value>(&cache_key) {
                    if !cached.is_undefined() {
                        return Ok(cached);
                    }
                }
            }

            let current_val = node_get.get(&prop_get);
            let _t2d = match current_val.try_to::<Transform2D>() {
                Ok(v) => v,
                Err(_) => {
                    return Err(ctx
                        .throw("Transform2D proxy: cannot read Godot Transform2D".into_js(&ctx)?));
                }
            };

            let col_idx = match prop.as_str() {
                "x" => Some(0u8),
                "y" => Some(1u8),
                "origin" => Some(2u8),
                _ => None,
            };

            match col_idx {
                Some(idx) => with_cache(&target, &col_cache_key(&prop), || {
                    make_col_proxy(&ctx, node_get.clone(), prop_get.clone(), idx)
                }),
                None => Ok(js::Value::new_undefined(ctx.clone())),
            }
        },
    )?;

    let node_set = gdnode;
    let prop_set = prop_name;
    let set = js::Function::new(
        ctx.clone(),
        move |ctx: js::Ctx<'js>,
              _target: js::Object<'js>,
              prop: String,
              val: js::Value<'js>|
              -> bool {
            let alive = node_set.is_instance_valid();
            if let Err(_) = gd_alive_handle(&ctx, alive) {
                return false;
            }
            let current_val = node_set.get(&prop_set);
            let Ok(mut t2d) = current_val.try_to::<Transform2D>() else {
                let _ = ctx.throw(
                    js::String::from_str(
                        ctx.clone(),
                        "Transform2D proxy: cannot read Godot Transform2D for mutation",
                    )
                    .unwrap()
                    .into_value(),
                );
                return false;
            };

            let col = match extract_col_from_val(&val) {
                Some(c) => c,
                None => return false,
            };

            match prop.as_str() {
                "x" => t2d.a = col,
                "y" => t2d.b = col,
                "origin" => t2d.origin = col,
                _ => {
                    let _ = ctx.throw(
                        js::String::from_str(
                            ctx.clone(),
                            format!("Transform2D proxy: unknown property '{}'", prop).as_str(),
                        )
                        .unwrap()
                        .into_value(),
                    );
                    return false;
                }
            }
            let mut node_set = node_set.clone();
            node_set.set(&prop_set, &Variant::from(t2d));
            true
        },
    )?;

    t2d_handler.set("get", get)?;
    t2d_handler.set("set", set)?;

    let proxy = js::Proxy::new(
        ctx.clone(),
        t2d_target,
        js::proxy::ProxyHandler::from_object(t2d_handler)?,
    )?;
    Ok(proxy.into_value())
}

fn make_col_proxy<'js>(
    ctx: &js::Ctx<'js>,
    gdnode: Gd<Node>,
    prop_name: StringName,
    col_idx: u8,
) -> js::Result<js::Value<'js>> {
    let target = js::Object::new(ctx.clone())?;
    let handler = js::Object::new(ctx.clone())?;

    let node_get = gdnode.clone();
    let prop_get = prop_name.clone();
    let get = js::Function::new(
        ctx.clone(),
        move |ctx: js::Ctx<'js>,
              _target: js::Object<'js>,
              field: String|
              -> js::Result<js::Value<'js>> {
            let alive = node_get.is_instance_valid();
            if field == "is_alive" {
                return Ok(js::Value::new_bool(ctx.clone(), alive));
            }
            gd_alive_handle(&ctx, alive)?;

            let current_val = node_get.get(&prop_get);
            let t2d = match current_val.try_to::<Transform2D>() {
                Ok(v) => v,
                Err(_) => {
                    return Err(ctx.throw(
                        "Transform2D column proxy: cannot read Transform2D".into_js(&ctx)?,
                    ));
                }
            };

            let col = match col_idx {
                0 => t2d.a,
                1 => t2d.b,
                _ => t2d.origin,
            };
            let res = match field.as_str() {
                "x" => col.x,
                "y" => col.y,
                _ => 0.0,
            };
            Ok(js::Value::new_number(ctx, res as f64))
        },
    )?;

    let node_set = gdnode;
    let prop_set = prop_name;
    let set = js::Function::new(
        ctx.clone(),
        move |ctx: js::Ctx<'js>, _target: js::Object<'js>, field: String, val: f32| -> bool {
            let alive = node_set.is_instance_valid();
            if let Err(_) = gd_alive_handle(&ctx, alive) {
                return false;
            }
            let current_val = node_set.get(&prop_set);
            let Ok(mut t2d) = current_val.try_to::<Transform2D>() else {
                let _ = ctx.throw(
                    js::String::from_str(
                        ctx.clone(),
                        "Transform2D column proxy: cannot read for mutation",
                    )
                    .unwrap()
                    .into_value(),
                );
                return false;
            };

            let col = match col_idx {
                0 => &mut t2d.a,
                1 => &mut t2d.b,
                _ => &mut t2d.origin,
            };
            match field.as_str() {
                "x" => col.x = val,
                "y" => col.y = val,
                _ => {
                    let _ = ctx.throw(
                        js::String::from_str(
                            ctx.clone(),
                            format!("Transform2D column proxy: unknown field '{}'", field).as_str(),
                        )
                        .unwrap()
                        .into_value(),
                    );
                    return false;
                }
            }
            let mut node_set = node_set.clone();
            node_set.set(&prop_set, &Variant::from(t2d));
            true
        },
    )?;

    handler.set("get", get)?;
    handler.set("set", set)?;

    let proxy = js::Proxy::new(
        ctx.clone(),
        target,
        js::proxy::ProxyHandler::from_object(handler)?,
    )?;
    Ok(proxy.into_value())
}

fn extract_col_from_val(val: &js::Value<'_>) -> Option<Vector2> {
    let obj = val.as_object()?;
    let x: f32 = obj.get("x").ok()?;
    let y: f32 = obj.get("y").ok()?;
    Some(Vector2::new(x, y))
}
