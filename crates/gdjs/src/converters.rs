use crate::prelude::*;
use js_core::vectors::*;

pub fn js_to_godot_variant(_ctx: &js::Ctx<'_>, val: js::Value<'_>) -> Variant {
    if val.is_string() {
        if let Some(js_str) = val.as_string() {
            if let Ok(rust_str) = js_str.to_string() {
                return Variant::from(GString::from(rust_str.as_str()));
            }
        }
    } else if val.is_number() {
        if let Some(js_num) = val.as_number() {
            return Variant::from(js_num);
        }
    } else if val.is_bool() {
        if let Some(js_bool) = val.as_bool() {
            return Variant::from(js_bool);
        }
    } else if let Some(obj) = val.as_object() {
        if let Some(v2_class) = js::class::Class::<JsVector2>::from_object(&obj) {
            let internal = v2_class.borrow();
            return Variant::from(godot::prelude::Vector2::new(internal.x, internal.y));
        }
    }
    Variant::nil()
}

pub fn godot_variant_to_js<'js, F>(
    ctx: &js::Ctx<'js>,
    variant: Variant,
    create_proxy: &mut F,
) -> js::Result<js::Value<'js>>
where
    F: FnMut(&js::Ctx<'js>, Gd<godot::prelude::Object>) -> js::Result<js::Value<'js>>,
{
    match variant.get_type() {
        VariantType::NIL => Ok(js::Value::new_null(ctx.clone())),

        VariantType::BOOL => {
            let b = variant.try_to::<bool>().unwrap_or(false);
            Ok(js::Value::new_bool(ctx.clone(), b))
        }

        VariantType::INT => {
            let val_i64 = variant.try_to::<i64>().unwrap_or(0);

            if let Ok(val_i32) = i32::try_from(val_i64) {
                Ok(js::Value::new_int(ctx.clone(), val_i32))
            } else {
                Ok(js::Value::new_float(ctx.clone(), val_i64 as f64))
            }
        }

        VariantType::FLOAT => {
            let f = variant.try_to::<f64>().unwrap_or(0.0);
            Ok(js::Value::new_float(ctx.clone(), f))
        }

        VariantType::STRING | VariantType::STRING_NAME => {
            let s = variant.to_string(); // Безопасно для String и StringName
            let js_str = js::String::from_str(ctx.clone(), &s)?;
            Ok(js_str.into_value())
        }

        VariantType::VECTOR2 => {
            let v2 = variant.try_to::<Vector2>().unwrap_or(Vector2::ZERO);
            let js_vec = JsVector2 { x: v2.x, y: v2.y };
            let class_instance = js::Class::instance(ctx.clone(), js_vec)?;
            Ok(class_instance.into_value())
        }

        VariantType::ARRAY => {
            let godot_array = variant.try_to::<VarArray>().unwrap_or_default();
            let js_array = js::Array::new(ctx.clone())?;

            for (i, item) in godot_array.iter_shared().enumerate() {
                let js_item = godot_variant_to_js(ctx, item, create_proxy)?;
                js_array.set(i, js_item)?;
            }
            Ok(js_array.into_value())
        }

        VariantType::DICTIONARY => {
            let godot_dict = variant
                .try_to::<Dictionary<Variant, Variant>>()
                .unwrap_or_default();
            let js_obj = js::object::Object::new(ctx.clone())?;

            for (key, value) in godot_dict.iter_shared() {
                let key_str = key.to_string();
                let js_value = godot_variant_to_js(ctx, value, create_proxy)?;
                js_obj.set(&key_str, js_value)?;
            }
            Ok(js_obj.into_value())
        }

        VariantType::OBJECT => {
            if let Ok(gd_obj) = variant.try_to::<Gd<Object>>() {
                if !gd_obj.is_instance_valid() {
                    return Ok(js::Value::new_undefined(ctx.clone()));
                }

                create_proxy(ctx, gd_obj)
            } else {
                Ok(js::Value::new_undefined(ctx.clone()))
            }
        }

        _ => {
            godot_warn!(
                "Bifrost Marshalling: Unsupported VariantType {:?}",
                variant.get_type()
            );
            Ok(js::Value::new_undefined(ctx.clone()))
        }
    }
}

pub fn js_to_gd_args<'js>(ctx: &js::Ctx<'js>, args: js::prelude::Rest<js::Value<'js>>) -> Vec<Variant> {
    let mut godot_args = Vec::with_capacity(args.0.len());
    for arg in args.0 {
        godot_args.push(js_to_godot_variant(ctx, arg));
    }
    godot_args
}
