use crate::manager::JsRuntimeManager;
use crate::manager_context::JsSignalMeta;
use crate::prelude::*;
use crate::proxy_deps::ProxyDeps;
use gdjs::converters::{godot_variant_to_js, js_to_gd_args, js_to_godot_variant};
use gdjs::proxy_rect::create_rect2_proxy;
use gdjs::proxy_transform::create_transform2d_proxy;
use gdjs::proxy_vec::create_vector2_proxy;
use gdjs::util::{check_alive_handle, gd_alive_handle};
use gdjs::util::{cache_fn_key, cache_key, get_cached, with_cache};
use js_core::utils::extract_trace;

use js_core::js::IntoJs;

#[derive(GodotClass)]
#[class(base=Node)]
pub struct JsNode {
    base: Base<Node>,

    #[export]
    script_path: GString,

    manager: Option<Gd<JsRuntimeManager>>,
}

#[godot_api]
impl JsNode {
    #[func]
    pub fn register_signal(&mut self, signal_name: String) {
        if self.base().has_user_signal(&signal_name) {
            godot_error!(
                "Bifrost Warn: signal with the name '{}' already registered",
                signal_name
            );
            return;
        }

        println!("Register {signal_name}");
        self.to_gd().add_user_signal(&signal_name);
    }

    #[func]
    pub fn emit_signal(&mut self, signal_name: String, args: Array<Variant>) {
        let signal_string_name = StringName::from(&signal_name);

        if !self.base().has_user_signal(&signal_string_name) {
            godot_error!(
                "Bifrost Error: Try to emit unregistered signal '{}'",
                signal_name
            );
            return;
        }

        let variant_vec: Vec<Variant> = args.iter_shared().collect();

        println!("Emit {signal_name}");
        self.to_gd().emit_signal(&signal_string_name, &variant_vec);
    }
}

#[godot_api]
impl INode for JsNode {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            script_path: GString::from(""),
            manager: None,
        }
    }

    fn process(&mut self, _: f64) {
        if self.script_path.is_empty() {
            return;
        }
        let instance_id = self.base().instance_id();
        if let Some(manager) = self.manager.as_mut() {
            manager.bind_mut().enqueue_process(instance_id);
        }
    }

    fn ready(&mut self) {
        if self.script_path.is_empty() {
            godot_warn!("JsNode [{}] has no filled path!", self.base().get_name());
            return;
        }

        let mut current_parent = self.base().get_parent();
        let mut found_manager: Option<Gd<JsRuntimeManager>> = None;
        while let Some(parent) = current_parent {
            match parent.try_cast::<JsRuntimeManager>() {
                Ok(manager_gd) => {
                    found_manager = Some(manager_gd);
                    break;
                }
                Err(returned_parent) => {
                    current_parent = returned_parent.get_parent();
                }
            }
        }

        if let Some(mut manager) = found_manager {
            self.manager = Some(manager.clone());
            manager.call_deferred(
                "register_js_node",
                &[
                    self.base().to_godot().to_variant(),
                    self.script_path.to_variant(),
                ],
            );
        }
    }

    fn exit_tree(&mut self) {
        let id = self.base().instance_id();
        if let Some(manager) = self.manager.as_mut() {
            manager.bind_mut().unregister_js_node(id);
        }
    }
}

#[derive(js::JsLifetime)]
pub struct JsNodeProxy {
    #[allow(dead_code)]
    pub godot_node: Gd<godot::prelude::Object>,
}

impl<'js> js::class::Trace<'js> for JsNodeProxy {
    fn trace<'a>(&self, _tracer: js::class::Tracer<'a, 'js>) {}
}

impl<'js> js::IntoJs<'js> for JsNodeProxy {
    fn into_js(self, ctx: &js::Ctx<'js>) -> js::Result<js::Value<'js>> {
        let obj = js::Object::new(ctx.clone())?;
        Ok(obj.into_value())
    }
}

pub fn create_godot_js_proxy<'js>(deps: &ProxyDeps<'js>) -> js::Result<js::Value<'js>> {
    let target = JsNodeProxy {
        godot_node: deps.node.clone(),
    };
    let target_js = target.into_js(&deps.ctx)?;
    let obj = target_js.as_object().unwrap();
    obj.set("gd_instance_id", deps.node.instance_id().to_i64() as f64)?;
    let _ = obj;

    let handler = js::Object::new(deps.ctx.clone())?;
    handler.set("get", make_get_trap(deps)?)?;
    handler.set("set", make_set_trap(deps)?)?;

    let proxy = js::Proxy::new(
        deps.ctx.clone(),
        target_js,
        js::proxy::ProxyHandler::from_object(handler)?,
    )?;
    Ok(proxy.into_value())
}

fn make_get_trap<'js>(deps: &ProxyDeps<'js>) -> js::Result<js::Function<'js>> {
    let node = deps.node.clone();
    let man_ctx = deps.manager_ctx.clone();
    js::Function::new(
        deps.ctx.clone(),
        move |ctx: js::Ctx<'js>,
              target_obj: js::Object<'js>,
              prop: String|
              -> js::Result<js::Value<'js>> {
            let deps = ProxyDeps {
                ctx: ctx.clone(),
                node: node.clone(),
                manager_ctx: man_ctx.clone(),
            };

            if let Some(val) = get_cached(&target_obj, &prop)? {
                return Ok(val);
            }

            let alive = node.is_instance_valid();
            if prop == "is_alive" {
                return Ok(js::Value::new_bool(ctx.clone(), alive));
            }

            gd_alive_handle(&ctx, alive)?;

            let cache_prop_name = cache_key(&prop);

            if let Some(val) = get_special_property(&deps, &prop)? {
                return Ok(val);
            }

            let string_name = StringName::from(&prop);
            let prop_variant = node.get(&string_name);

            if prop_variant.try_to::<Vector2>().is_ok() {
                return with_cache(&target_obj, &cache_prop_name, || {
                    create_vector2_proxy(&ctx, node.clone(), string_name.clone())
                });
            }

            if prop_variant.try_to::<Rect2>().is_ok() {
                return with_cache(&target_obj, &cache_prop_name, || {
                    create_rect2_proxy(&ctx, node.clone(), string_name.clone())
                });
            }

            if prop_variant.try_to::<Transform2D>().is_ok() {
                return with_cache(&target_obj, &cache_prop_name, || {
                    create_transform2d_proxy(&ctx, node.clone(), string_name.clone())
                });
            }

            if node.has_method(&string_name) {
                return handle_method_call(&deps, &target_obj, &prop, string_name);
            }

            let godot_variant = node.get(&string_name);
            let mut create_proxy =
                |ctx: &js::Ctx<'js>, v: Gd<godot::prelude::Object>| -> js::Result<js::Value<'js>> {
                    let new_deps = ProxyDeps {
                        ctx: ctx.clone(),
                        node: v,
                        manager_ctx: man_ctx.clone(),
                    };
                    create_godot_js_proxy(&new_deps)
                };
            godot_variant_to_js(&ctx, godot_variant, &mut create_proxy)
        },
    )
}

#[inline(always)]
fn make_cache_fn_name(prop: &str) -> String {
    cache_fn_key(prop)
}

fn handle_method_call<'js>(
    deps: &ProxyDeps<'js>,
    target_obj: &js::Object<'js>,
    prop: &str,
    string_name: StringName,
) -> js::Result<js::Value<'js>> {
    let cache_fn_name = make_cache_fn_name(prop);
    let cached_fn: js::Value<'js> = target_obj.get(&cache_fn_name)?;
    if !cached_fn.is_undefined() && !cached_fn.is_null() {
        return Ok(cached_fn);
    }

    let method_value = match get_special_method(deps, prop)? {
        Some(v) => v,
        None => make_godot_method_fn(deps, string_name)?.into_value(),
    };
    target_obj.set(&cache_fn_name, method_value.clone())?;
    Ok(method_value)
}

fn make_godot_method_fn<'js>(
    deps: &ProxyDeps<'js>,
    method_name: StringName,
) -> js::Result<js::Function<'js>> {
    let node = deps.node.clone();
    let man_ctx = deps.manager_ctx.clone();
    js::Function::new(
        deps.ctx.clone(),
        move |ctx: js::Ctx<'js>,
              args: js::prelude::Rest<js::Value<'js>>|
              -> js::Result<js::Value<'js>> {
            check_alive_handle(&ctx, &node)?;

            let result_variant = node.clone().call(&method_name, &js_to_gd_args(args)?);
            let mut create_proxy = |ctx: &js::Ctx<'js>,
                                    obj: Gd<godot::prelude::Object>|
             -> js::Result<js::Value<'js>> {
                let child_deps = ProxyDeps {
                    ctx: ctx.clone(),
                    node: obj,
                    manager_ctx: man_ctx.clone(),
                };
                create_godot_js_proxy(&child_deps)
            };
            godot_variant_to_js(&ctx, result_variant, &mut create_proxy)
        },
    )
}

fn get_special_method<'js>(
    deps: &ProxyDeps<'js>,
    prop: &str,
) -> js::Result<Option<js::Value<'js>>> {
    Ok(Some(match prop {
        "is_class" => make_is_class(&deps.ctx, &deps.node)?.into_value(),
        "connect" => make_connect(deps)?.into_value(),
        "disconnect" => make_disconnect(deps)?.into_value(),
        _ => return Ok(None),
    }))
}

fn make_disconnect<'js>(deps: &ProxyDeps<'js>) -> js::Result<js::Function<'js>> {
    let context_weak = deps.manager_ctx.downgrade();
    let node = deps.node.clone();
    let connect_fn = js::Function::new(
        deps.ctx.clone(),
        move |ctx: js::Ctx<'js>, callback_id: u64| -> js::Result<()> {
            check_alive_handle(&ctx, &node)?;
            if let Some((meta, context)) = context_weak.upgrade().and_then(|context| {
                let meta = context.borrow_mut().drop_callback(callback_id)?;
                Some((meta, context))
            }) {
                if meta.node_id == node.instance_id() {
                    let mut node = node.clone();
                    node.disconnect(&meta.signal_name, &meta.callable);
                } else {
                    context.borrow_mut().save_callback(meta.id, meta);
                    godot_warn!(
                        "{}\n{}",
                        format!(
                            "Incorrect node disconnect. Node({}) has no callback with id: {}",
                            node.clone()
                                .try_cast::<Node>()
                                .ok()
                                .map(|n| n.get_name().to_string())
                                .unwrap_or_default(),
                            callback_id
                        ),
                        extract_trace(&ctx)
                    );
                }
            }
            Ok(())
        },
    )?;
    Ok(connect_fn)
}

fn make_connect<'js>(deps: &ProxyDeps<'js>) -> js::Result<js::Function<'js>> {
    let context_weak = deps.manager_ctx.downgrade();
    let node = deps.node.clone();
    let connect_fn = js::Function::new(
        deps.ctx.clone(),
        move |ctx: js::Ctx<'js>, signal_name: String, callback: js::Function<'js>| {
            check_alive_handle(&ctx, &node)?;
            if let Some(context) = context_weak.upgrade() {
                let id = context.borrow_mut().next_callback_id();
                let ctx_weak = context_weak.clone();
                let callable = Callable::from_linked_fn(
                    "js_signal_handler",
                    &node,
                    move |args: &[&godot::prelude::Variant]| {
                        if let Some(context) = ctx_weak.upgrade() {
                            context.borrow_mut().enqueue(id, args);
                        }

                        godot::prelude::Variant::nil()
                    },
                );

                context.borrow_mut().save_callback(
                    id,
                    JsSignalMeta {
                        id,
                        callback: js::Persistent::save(&ctx, callback),
                        callable: callable.clone(),
                        node_id: node.instance_id(),
                        signal_name: signal_name.clone(),
                    },
                );
                let mut node = node.clone();
                println!("Connect on {signal_name}");
                node.connect(&StringName::from(&signal_name), &callable);
                Ok(id)
            } else {
                Err(ctx.throw("Failed to get context".into_js(&ctx)?))
            }
        },
    )?;
    Ok(connect_fn)
}

fn get_special_property<'js>(
    deps: &ProxyDeps<'js>,
    prop: &str,
) -> js::Result<Option<js::Value<'js>>> {
    Ok(Some(match prop {
        "name" => {
            let name_str = deps
                .try_node()
                .map(|n| n.get_name().to_string())
                .unwrap_or_default();
            js::String::from_str(deps.ctx.clone(), name_str.as_str())
                .map(|js_s| js_s.into_value())?
        }
        "class_type" => {
            let class_name = deps.node.get_class().to_string();
            let js_str = js::String::from_str(deps.ctx.clone(), class_name.as_str())?;
            js_str.into_value()
        }
        "id" => js::Value::new_number(deps.ctx.clone(), deps.node.instance_id().to_i64() as f64),
        "parent" => match deps.try_node().and_then(|n| n.get_parent()) {
            Some(parent) => return create_godot_js_proxy(&deps.with_node(parent)).map(Some),
            None => js::Value::new_null(deps.ctx.clone()),
        },
        _ => return Ok(None),
    }))
}

fn make_is_class<'js>(
    ctx: &js::Ctx<'js>,
    node: &Gd<godot::prelude::Object>,
) -> js::Result<js::Function<'js>> {
    let node = node.clone();
    let is_class_fn = js::Function::new(
        ctx.clone(),
        move |ctx: js::Ctx<'js>, class_name: String| -> js::Result<bool> {
            check_alive_handle(&ctx, &node)?;

            let is_inherited = node.is_class(&class_name);
            Ok(is_inherited)
        },
    )?;

    Ok(is_class_fn)
}

fn make_set_trap<'js>(deps: &ProxyDeps<'js>) -> js::Result<js::Function<'js>> {
    let node = deps.node.clone();
    js::Function::new(
        deps.ctx.clone(),
        move |ctx: js::Ctx<'js>,
              _target_obj: js::Object<'js>,
              prop: String,
              value: js::Value<'js>|
              -> bool {
            if prop == "gd_instance_id" {
                return false;
            }
            let alive = node.is_instance_valid();
            if let Err(err) = gd_alive_handle(&ctx, alive) {
                godot_error!("{}", err);
                return false;
            }
            let string_name = StringName::from(&prop);

            if node.get(&string_name).try_to::<Vector2>().is_ok() {
                godot_error!("Mutate of {} is forbidden\n{}", prop, extract_trace(&ctx));
                return false;
            }
            if node.get(&string_name).try_to::<Rect2>().is_ok() {
                godot_error!("Mutate of {} is forbidden\n{}", prop, extract_trace(&ctx));
                return false;
            }
            if node.get(&string_name).try_to::<Transform2D>().is_ok() {
                godot_error!("Mutate of {} is forbidden\n{}", prop, extract_trace(&ctx));
                return false;
            }
            let godot_variant = match js_to_godot_variant(value) {
                Ok(v) => v,
                Err(err) => {
                    godot_error!("js_to_godot_variant error: {:?}", err);
                    return false;
                }
            };

            let mut godot_node_cloned_set = node.clone();
            godot_node_cloned_set.set(&string_name, &godot_variant);
            true
        },
    )
}
