use godot::prelude::*;
use js_core::vectors::*;
use rquickjs::{Class, Ctx, Value, prelude::Rest};

pub fn js_to_godot_variant(_ctx: &Ctx<'_>, val: Value<'_>) -> Variant {
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
        if let Some(v2_class) = rquickjs::class::Class::<JsVector2>::from_object(&obj) {
            let internal = v2_class.borrow();
            return Variant::from(godot::prelude::Vector2::new(internal.x, internal.y));
        }
    }
    Variant::nil()
}

pub fn godot_variant_to_js<'js>(ctx: &Ctx<'js>, variant: Variant) -> rquickjs::Result<Value<'js>> {
    match variant.get_type() {
        VariantType::VECTOR2 => {
            let v2 = variant.try_to::<Vector2>().unwrap();
            let js_vec = JsVector2 { x: v2.x, y: v2.y };

            let class_instance = Class::instance(ctx.clone(), js_vec)?;
            Ok(class_instance.into_value())
        }
        VariantType::BOOL => Ok(Value::new_bool(
            ctx.clone(),
            variant.try_to::<bool>().unwrap(),
        )),
        VariantType::INT => Ok(Value::new_int(
            ctx.clone(),
            variant.try_to::<i32>().unwrap(),
        )),
        VariantType::FLOAT => Ok(Value::new_float(
            ctx.clone(),
            variant.try_to::<f64>().unwrap(),
        )),
        VariantType::STRING => {
            let s = variant.try_to::<String>().unwrap();
            let js_str = rquickjs::String::from_str(ctx.clone(), &s)?;
            Ok(js_str.into_value())
        }
        _ => Ok(Value::new_undefined(ctx.clone())),
    }
}

pub fn js_to_gd_args<'js>(ctx: &Ctx<'js>, args: Rest<Value<'js>>) -> Vec<Variant> {
    let mut godot_args = Vec::with_capacity(args.0.len());
    for arg in args.0 {
        godot_args.push(js_to_godot_variant(ctx, arg));
    }
    godot_args
}
