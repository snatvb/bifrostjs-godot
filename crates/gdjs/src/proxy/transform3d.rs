use godot::prelude::*;
use js_core::js::{self, IntoJs};

use crate::util::{col_cache_key, gd_alive_handle, peek_cache, with_cache};

pub fn create<'js>(
    ctx: &js::Ctx<'js>,
    gdobject: Gd<godot::prelude::Object>,
    prop_name: StringName,
) -> js::Result<js::Value<'js>> {
    let gdnode = match gdobject.try_cast::<Node>() {
        Ok(n) => n,
        Err(_) => {
            return Err(ctx.throw("Transform3D proxy: not a Node".into_js(ctx)?));
        }
    };
    let t3d_target = js::Object::new(ctx.clone())?;
    let t3d_handler = js::Object::new(ctx.clone())?;

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
            if let Some(cached) = peek_cache(&target, &cache_key)? {
                return Ok(cached);
            }

            let current_val = node_get.get(&prop_get);
            let _t3d = match current_val.try_to::<Transform3D>() {
                Ok(v) => v,
                Err(_) => {
                    return Err(ctx
                        .throw("Transform3D proxy: cannot read Godot Transform3D".into_js(&ctx)?));
                }
            };

            let col_idx = match prop.as_str() {
                "x" => Some(0u8),
                "y" => Some(1u8),
                "z" => Some(2u8),
                "origin" => Some(3u8),
                _ => None,
            };

            match col_idx {
                Some(idx) => with_cache(&target, &cache_key, || {
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
            if gd_alive_handle(&ctx, alive).is_err() {
                return false;
            }
            let current_val = node_set.get(&prop_set);
            let Ok(mut t3d) = current_val.try_to::<Transform3D>() else {
                let _ = ctx.throw(
                    js::String::from_str(
                        ctx.clone(),
                        "Transform3D proxy: cannot read Godot Transform3D for mutation",
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
                "x" => t3d.basis.set_col_a(col),
                "y" => t3d.basis.set_col_b(col),
                "z" => t3d.basis.set_col_c(col),
                "origin" => t3d.origin = col,
                _ => {
                    let _ = ctx.throw(
                        js::String::from_str(
                            ctx.clone(),
                            format!("Transform3D proxy: unknown property '{}'", prop).as_str(),
                        )
                        .unwrap()
                        .into_value(),
                    );
                    return false;
                }
            }
            let mut node_set = node_set.clone();
            node_set.set(&prop_set, &Variant::from(t3d));
            true
        },
    )?;

    t3d_handler.set("get", get)?;
    t3d_handler.set("set", set)?;

    let proxy = js::Proxy::new(
        ctx.clone(),
        t3d_target,
        js::proxy::ProxyHandler::from_object(t3d_handler)?,
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
            let t3d = match current_val.try_to::<Transform3D>() {
                Ok(v) => v,
                Err(_) => {
                    return Err(ctx.throw(
                        "Transform3D column proxy: cannot read Transform3D".into_js(&ctx)?,
                    ));
                }
            };

            let col = match col_idx {
                0 => t3d.basis.col_a(),
                1 => t3d.basis.col_b(),
                2 => t3d.basis.col_c(),
                _ => t3d.origin,
            };
            let res = match field.as_str() {
                "x" => col.x,
                "y" => col.y,
                "z" => col.z,
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
            if gd_alive_handle(&ctx, alive).is_ok() {
                return false;
            }
            let current_val = node_set.get(&prop_set);
            let Ok(mut t3d) = current_val.try_to::<Transform3D>() else {
                let _ = ctx.throw(
                    js::String::from_str(
                        ctx.clone(),
                        "Transform3D column proxy: cannot read for mutation",
                    )
                    .unwrap()
                    .into_value(),
                );
                return false;
            };

            match col_idx {
                0 => match field.as_str() {
                    "x" => t3d.basis.rows[0].x = val,
                    "y" => t3d.basis.rows[1].x = val,
                    "z" => t3d.basis.rows[2].x = val,
                    _ => {
                        let _ = ctx.throw(
                            js::String::from_str(
                                ctx.clone(),
                                format!("Transform3D column proxy: unknown field '{}'", field)
                                    .as_str(),
                            )
                            .unwrap()
                            .into_value(),
                        );
                        return false;
                    }
                },
                1 => match field.as_str() {
                    "x" => t3d.basis.rows[0].y = val,
                    "y" => t3d.basis.rows[1].y = val,
                    "z" => t3d.basis.rows[2].y = val,
                    _ => {
                        let _ = ctx.throw(
                            js::String::from_str(
                                ctx.clone(),
                                format!("Transform3D column proxy: unknown field '{}'", field)
                                    .as_str(),
                            )
                            .unwrap()
                            .into_value(),
                        );
                        return false;
                    }
                },
                2 => match field.as_str() {
                    "x" => t3d.basis.rows[0].z = val,
                    "y" => t3d.basis.rows[1].z = val,
                    "z" => t3d.basis.rows[2].z = val,
                    _ => {
                        let _ = ctx.throw(
                            js::String::from_str(
                                ctx.clone(),
                                format!("Transform3D column proxy: unknown field '{}'", field)
                                    .as_str(),
                            )
                            .unwrap()
                            .into_value(),
                        );
                        return false;
                    }
                },
                _ => match field.as_str() {
                    "x" => t3d.origin.x = val,
                    "y" => t3d.origin.y = val,
                    "z" => t3d.origin.z = val,
                    _ => {
                        let _ = ctx.throw(
                            js::String::from_str(
                                ctx.clone(),
                                format!("Transform3D column proxy: unknown field '{}'", field)
                                    .as_str(),
                            )
                            .unwrap()
                            .into_value(),
                        );
                        return false;
                    }
                },
            }
            let mut node_set = node_set.clone();
            node_set.set(&prop_set, &Variant::from(t3d));
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

fn extract_col_from_val(val: &js::Value<'_>) -> Option<Vector3> {
    let obj = val.as_object()?;
    let x: f32 = obj.get("x").ok()?;
    let y: f32 = obj.get("y").ok()?;
    let z: f32 = obj.get("z").ok()?;
    Some(Vector3::new(x, y, z))
}
