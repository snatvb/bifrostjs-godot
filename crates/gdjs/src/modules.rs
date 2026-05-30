use std::path::{Component, Path};

use crate::prelude::*;
use crate::util::load_js;
use godot::classes::ResourceUid;
use js::Module;
use js::loader::{Loader, Resolver};

pub struct JsResolver;
pub struct JsLoader;

fn normalize_virtual_path(path: &str) -> (Option<String>, String) {
    let (protocol, rest) = path.rsplit_once("://").unwrap_or(("", path));
    let mut segs = Vec::new();
    for c in Path::new(rest).components() {
        match c {
            Component::Normal(s) => segs.push(s.to_str().unwrap()),
            Component::ParentDir => {
                segs.pop();
            }
            _ => {}
        }
    }
    (
        (!protocol.is_empty()).then(|| protocol.to_string()),
        segs.join("/"),
    )
}

fn ensure_js_extension(name: &str) -> String {
    if name.ends_with(".js") {
        name.to_string()
    } else {
        format!("{}.js", name)
    }
}

impl Resolver for JsResolver {
    fn resolve<'js>(&mut self, _ctx: &js::Ctx<'js>, base: &str, name: &str) -> js::Result<String> {
        if name.starts_with(".") {
            let error = || js::Error::Resolving {
                base: base.to_string(),
                name: name.to_string(),
                message: Some(format!("Can't resolve path for {name} in {base}")),
            };
            let resolved = if base.starts_with("uid:") {
                ResourceUid::ensure_path(base).to_string()
            } else {
                base.to_string()
            };
            let base_dir = resolved
                .rsplit_once('/')
                .map(|(d, _)| d)
                .ok_or_else(error)?;
            let (protocol, normalized) = normalize_virtual_path(&format!("{base_dir}/{name}"));
            let protocol = protocol.map(|p| format!("{p}://"));
            return Ok(ensure_js_extension(&format!(
                "{}{}",
                protocol.unwrap_or_else(|| "".to_string()),
                normalized
            )));
        }

        Ok(name.to_string())
    }
}

impl Loader for JsLoader {
    fn load<'js>(
        &mut self,
        ctx: &js::Ctx<'js>,
        name: &str,
    ) -> js::Result<Module<'js, js::module::Declared>> {
        println!("Load {name}");
        let file = load_js(name).ok_or_else(|| js::Error::Loading {
            name: "JsLoader".to_string(),
            message: Some(format!("Failed to load load {name}")),
        })?;
        Module::declare(ctx.clone(), file.path, file.source)
    }
}
