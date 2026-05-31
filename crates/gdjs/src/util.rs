use std::io::Read;

use godot::classes::file_access::ModeFlags;
use godot::prelude::*;
use godot::tools::GFile;
use js_core::js::{self, IntoJs};

pub struct JsFile {
    pub source: String,
    pub path: String,
}

pub fn load_ts(path: &str) -> Result<JsFile, String> {
    let mut file = load_js(path)?;
    file.source = js_core::typescript::strip_types_fast_default(&file.source)?;
    Ok(file)
}

pub fn load_js(godot_path: &str) -> Result<JsFile, String> {
    let mut file = match GFile::open(godot_path, ModeFlags::READ) {
        Ok(f) => f,
        Err(err) => {
            return Err(format!(
                "Failed to load JS file [{}]: {:?}",
                godot_path, err
            ));
        }
    };

    let mut js_code = String::new();
    if let Err(err) = file.read_to_string(&mut js_code) {
        return Err(format!("Failed to read file [{}]: {:?}", godot_path, err));
    }

    Ok(JsFile {
        source: js_code,
        path: godot_path.to_string(),
    })
}

fn print_error(ctx: &js::Ctx<'_>, err: &js::Error) {
    if let js::Error::Exception = err {
        let error_value = ctx.catch();

        if let Some(exception) = error_value.as_exception() {
            let message = exception
                .message()
                .unwrap_or_else(|| "Unknown Error".to_string());
            godot_error!("JS Exception: {}", message);

            if let Some(stack) = exception.stack() {
                godot_error!("JS stack trace:\n{}", stack);
            }
        } else if let Some(js_str) = error_value.as_string() {
            if let Ok(rust_str) = js_str.to_string() {
                godot_error!("JS threw Error: {}", rust_str);
            }
        } else {
            godot_error!("JS threw unknown object");
        }
    } else {
        godot_error!("JS runtime Error: {:?}", err);
    }
}

pub fn handle_error<T>(ctx: &js::Ctx<'_>, res: &js::Result<T>) {
    if let Err(err) = res {
        print_error(ctx, err);
    }
}

pub fn with_handle_error<T, F: FnOnce(T)>(ctx: &js::Ctx<'_>, res: js::Result<T>, f: F) {
    match res {
        Ok(r) => f(r),
        Err(e) => print_error(ctx, &e),
    }
}

pub fn cache_key(prop: &str) -> String {
    format!("_cache_{}", prop)
}

pub fn cache_fn_key(prop: &str) -> String {
    format!("_cache_fn_{}", prop)
}

pub fn col_cache_key(prop: &str) -> String {
    format!("_col_cache_{}", prop)
}

pub fn with_cache<'js, F>(
    target: &js::Object<'js>,
    key: &str,
    factory: F,
) -> js::Result<js::Value<'js>>
where
    F: FnOnce() -> js::Result<js::Value<'js>>,
{
    if let Some(cached) = peek_cache(target, key)? {
        return Ok(cached);
    }
    let val = factory()?;
    target.set(key, val.clone())?;
    Ok(val)
}

pub fn get_cached<'js>(target: &js::Object<'js>, key: &str) -> js::Result<Option<js::Value<'js>>> {
    if target.contains_key(key).unwrap_or(false) {
        Ok(target.get::<_, js::Value>(key).ok())
    } else {
        Ok(None)
    }
}

pub fn peek_cache<'js>(target: &js::Object<'js>, key: &str) -> js::Result<Option<js::Value<'js>>> {
    if target.contains_key(key).unwrap_or(false)
        && let Ok(cached) = target.get::<_, js::Value>(key)
        && !cached.is_undefined()
    {
        return Ok(Some(cached));
    }
    Ok(None)
}

pub fn gd_alive_handle(ctx: &js::Ctx, alive: bool) -> js::Result<()> {
    if !alive {
        return Err(ctx.throw(
            "Cannot read property: Godot Node instance is already deleted or invalid!"
                .into_js(ctx)?,
        ));
    }
    Ok(())
}

pub fn check_alive_handle(ctx: &js::Ctx, gdnode: &Gd<Object>) -> js::Result<()> {
    gd_alive_handle(ctx, gdnode.is_instance_valid())
}
