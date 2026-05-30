use std::io::Read;

use godot::classes::file_access::ModeFlags;
use godot::tools::GFile;

use crate::prelude::*;

pub struct JsFile {
    pub source: String,
    pub path: String,
}

pub fn load_js(godot_path: &str) -> Option<JsFile> {
    let mut file = match GFile::open(godot_path, ModeFlags::READ) {
        Ok(f) => f,
        Err(err) => {
            godot_error!("Failed to load JS file [{}]: {:?}", godot_path, err);
            return None;
        }
    };

    let mut js_code = String::new();
    if let Err(err) = file.read_to_string(&mut js_code) {
        godot_error!("Failed to read file [{}]: {:?}", godot_path, err);
        return None;
    }

    Some(JsFile {
        source: js_code,
        path: godot_path.to_string(),
    })
}

pub fn with_handle_error<T, F: FnOnce(T)>(ctx: &Ctx<'_>, res: Result<T, rquickjs::Error>, f: F) {
    match res {
        Ok(r) => f(r),
        Err(e) => print_error(ctx, &e),
    }
}

pub fn print_error(ctx: &Ctx<'_>, err: &rquickjs::Error) {
    if let rquickjs::Error::Exception = err {
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

pub fn handle_error<T>(ctx: &Ctx<'_>, res: &Result<T, rquickjs::Error>) {
    if let Err(err) = res {
        print_error(ctx, err);
    }
}

// pub fn execute_js_script(file: &JsFile, ctx: &Ctx<'_>) {
//     let mut options = EvalOptions::default();
//     options.filename = Some(file.path.to_string());
//
//     let globals = ctx.globals();
//     let console_obj = rquickjs::Object::new(ctx.clone()).unwrap();
//     let log_fn = Function::new(ctx.clone(), console_log).unwrap();
//     console_obj.set("log", log_fn).unwrap();
//     globals.set("console", console_obj).unwrap();
//     handle_error(&ctx, &ctx.eval::<(), _>(file.source.as_str()));
// }
