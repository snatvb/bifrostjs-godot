use std::fmt::Debug;

use godot::{
    classes::{ClassDb, Input, ResourceLoader, resource_loader::CacheMode},
    global::Key,
};
use js::class::Trace;

use crate::{node::create_godot_js_proxy, prelude::*, proxy_deps::ProxyDeps};

#[derive(js::JsLifetime, Debug)]
#[js::class]
pub struct GodotJsEngine {
    ctx: ManagerCtxRef,
}

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

    #[qjs(rename = "createNode")]
    fn create_node<'js>(
        &self,
        ctx: js::Ctx<'js>,
        class_name: String,
    ) -> js::Result<js::Value<'js>> {
        let db = ClassDb::singleton();
        let node = db.instantiate(&StringName::from(&class_name));
        if let Ok(node) = node.try_to::<Gd<Node>>() {
            let deps = ProxyDeps {
                node: node.upcast(),
                ctx: ctx.clone(),
                manager_ctx: self.ctx.clone(),
            };
            return create_godot_js_proxy(&deps);
        }
        Ok(js::Undefined.into_value(ctx))
    }
    #[qjs(rename = "instantiate")]
    fn instantiate<'js>(&self, ctx: js::Ctx<'js>, path: String) -> js::Result<js::Value<'js>> {
        let mut loader = ResourceLoader::singleton();
        let resource: Gd<Resource> = loader
            .load_ex(&GString::from(&path))
            .type_hint("PackedScene")
            .cache_mode(CacheMode::REUSE)
            .done()
            .ok_or_else(|| js::Error::Loading {
                name: "LoaderLoad".to_string(),
                message: format!("Failed to load scene: {path}").into(),
            })?;
        let packed: Gd<PackedScene> = resource.try_cast().map_err(|_| js::Error::Loading {
            name: "LoaderLoad".to_string(),
            message: format!("Resource is not a PackedScene: {path}").into(),
        })?;
        let node: Gd<Node> = packed.instantiate().ok_or_else(|| js::Error::Loading {
            name: "LoaderLoad".to_string(),
            message: format!("Failed to instantiate scene: {path}").into(),
        })?;
        let deps = ProxyDeps {
            node: node.upcast(),
            ctx: ctx.clone(),
            manager_ctx: self.ctx.clone(),
        };
        create_godot_js_proxy(&deps)
    }
}

pub fn create_engine<'js>(ctx: &js::Ctx<'js>) -> js::Result<js::Value<'js>> {
    let manager_ctx = ctx
        .userdata::<ManagerCtxRef>()
        .expect("Manager context must be initialized");
    let engine = js::Class::instance(
        ctx.clone(),
        GodotJsEngine {
            ctx: manager_ctx.clone(),
        },
    )?;
    let input = js::Class::instance(ctx.clone(), gdjs::input::JsInput {})?;
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
