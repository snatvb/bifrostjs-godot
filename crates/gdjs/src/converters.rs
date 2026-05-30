use crate::prelude::*;
use js_core::vectors::*;

pub fn js_to_godot_variant(val: js::Value<'_>) -> js::Result<Variant> {
    match val.type_of() {
        js::Type::Null | js::Type::Undefined => Ok(Variant::nil()),

        js::Type::Bool => Ok(Variant::from(val.as_bool().unwrap_or(false))),

        js::Type::Int => Ok(Variant::from(val.as_int().unwrap_or(0))),

        js::Type::Float => Ok(Variant::from(val.as_number().unwrap_or(0.0))),

        js::Type::String => {
            let js_str = val.as_string().unwrap();
            let rust_str = js_str.to_string().unwrap_or_default();
            Ok(Variant::from(GString::from(rust_str.as_str())))
        }

        js::Type::BigInt => {
            let bi = val.as_big_int().unwrap();
            let n = bi.clone().to_i64().unwrap_or(0);
            Ok(Variant::from(n))
        }

        js::Type::Array => {
            let js_array = val.as_array().unwrap();
            let len = js_array.len();
            let mut gd_array = VarArray::new();
            gd_array.resize(len, &Variant::nil());
            for i in 0..len {
                let item: js::Value = js_array.get(i)?;
                let gd_item = js_to_godot_variant(item)?;
                gd_array.set(i, &gd_item);
            }
            Ok(Variant::from(gd_array))
        }

        js::Type::Object => {
            let obj = val.as_object().unwrap();
            if let Some(v2_class) = js::class::Class::<JsVector2>::from_object(obj) {
                let internal = v2_class.borrow();
                return Ok(Variant::from(godot::prelude::Vector2::new(
                    internal.x, internal.y,
                )));
            }
            let mut gd_dict = Dictionary::<Variant, Variant>::new();
            let keys: Vec<String> = obj.keys().collect::<js::Result<Vec<_>>>()?;
            for key in keys {
                let js_value: js::Value = obj.get(&key)?;
                let gd_value = js_to_godot_variant(js_value)?;
                gd_dict.set(&Variant::from(key), &gd_value);
            }
            Ok(Variant::from(gd_dict))
        }

        _ => {
            godot_warn!(
                "Bifrost Marshalling: Unsupported JS type '{:?}'",
                val.type_name()
            );
            Ok(Variant::nil())
        }
    }
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
            let s = variant.to_string();
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

pub fn js_to_gd_args<'js>(args: js::prelude::Rest<js::Value<'js>>) -> js::Result<Vec<Variant>> {
    let mut godot_args = Vec::with_capacity(args.0.len());
    for arg in args.0 {
        godot_args.push(js_to_godot_variant(arg)?);
    }
    Ok(godot_args)
}
