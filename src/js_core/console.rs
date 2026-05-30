use super::utils::*;
use crate::prelude::*;

pub fn log(_ctx: Ctx<'_>, Rest(args): Rest<Value<'_>>) {
    godot_print!("{}", convert_to_string(args.as_slice()));
}

pub fn trace(ctx: Ctx<'_>, Rest(args): Rest<Value<'_>>) {
    godot_print!(
        "{}\n{}",
        convert_to_string(args.as_slice()),
        extract_trace(&ctx)
    );
}

pub fn error(ctx: Ctx<'_>, Rest(args): Rest<Value<'_>>) {
    let error_msg = convert_to_string(args.as_slice());
    let mut stack_trace = String::new();

    if let Some(js_stack) = args
        .first()
        .filter(|x| x.is_object())
        .and_then(|x| x.as_object())
        .and_then(|x| x.get::<_, String>("stack").ok())
    {
        stack_trace = js_stack;
    }

    if stack_trace.is_empty() {
        stack_trace = extract_trace(&ctx);
    }

    godot_error!("Error: {}\n{}", error_msg, stack_trace);
}

pub fn create<'a>(ctx: &Ctx<'a>) -> rquickjs::Result<Object<'a>> {
    let console_obj = rquickjs::Object::new(ctx.clone())?;
    console_obj.set("log", Function::new(ctx.clone(), log)?)?;
    console_obj.set("trace", Function::new(ctx.clone(), trace)?)?;
    console_obj.set("error", Function::new(ctx.clone(), error)?)?;
    Ok(console_obj)
}
