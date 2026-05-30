use godot::{
    classes::{DisplayServer, Input},
    global::Key,
};
use js::class::Trace;

use crate::{input::JsInput, prelude::*};

#[derive(js::JsLifetime, Debug)]
#[js::class]
pub struct GodotJsEngine {}

impl<'js> Trace<'js> for GodotJsEngine {
    fn trace<'a>(&self, _tracer: js::class::Tracer<'a, 'js>) {}
}

#[js::methods]
impl GodotJsEngine {
    #[qjs(rename = "isKeyPressed")]
    fn is_key_pressed(&self, key: i32) -> bool {
        let gd_key = Key::try_from_ord(key).unwrap_or(Key::NONE);
        Input::singleton().is_key_pressed(gd_key)
    }
}

pub fn create_engine<'js>(ctx: &js::Ctx<'js>) -> js::Result<js::Value<'js>> {
    let engine = js::Class::instance(ctx.clone(), GodotJsEngine {})?;
    let input = js::Class::instance(ctx.clone(), JsInput {})?;
    let keys = js::Object::new(ctx.clone())?;
    keys.set("W", 87)?;
    keys.set("A", 65)?;
    keys.set("S", 83)?;
    keys.set("D", 68)?;
    {
        let obj = engine.as_object().unwrap();
        obj.set("input", input)?;
        obj.set("Keys", keys)?;
    }
    Ok(engine.into_value())
}
