use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::ops::Deref;
use std::rc::{Rc, Weak};

use crate::node::create_godot_js_proxy;
use crate::proxy_deps::ProxyDeps;
use gdjs::converters::godot_variant_to_js;
use godot::classes::class_macros::private::virtuals::Os::{Callable, Variant};
use godot::global::godot_error;
use godot::obj::InstanceId;
use godot::prelude::Gd;
use rquickjs::function::Rest;
use rquickjs::{Ctx, Function, Persistent, Value};

#[derive(Debug)]
pub struct JsSignalMeta {
    pub callback: Persistent<Function<'static>>,
    pub callable: Callable,
    pub id: u64,
    pub node_id: InstanceId,
    pub signal_name: String,
}

#[derive(Debug)]
pub struct FiredSignal {
    pub id: u64,
    pub arguments: Vec<Variant>,
}

#[derive(Default, Debug)]
pub struct JsManagerContext {
    last_id: u64,
    signal_registry: HashMap<u64, JsSignalMeta>,
    // signal_queue: VecDeque<u64>,
    signal_queue: VecDeque<FiredSignal>,
}

impl JsManagerContext {
    pub fn clear(&mut self) {
        self.last_id = 0;
        self.signal_registry.clear();
        self.signal_queue.clear();
    }

    pub fn next_callback_id(&mut self) -> u64 {
        self.last_id += 1;
        self.last_id
    }

    pub fn save_callback(&mut self, id: u64, meta: JsSignalMeta) {
        self.signal_registry.insert(id, meta);
    }

    pub fn drop_callback(&mut self, id: u64) -> Option<JsSignalMeta> {
        self.signal_registry.remove(&id)
    }

    pub fn remove_callback(&mut self, id: u64) {
        self.signal_registry.remove(&id);
    }

    pub fn enqueue(&mut self, id: u64, arguments: &[&godot::prelude::Variant]) {
        self.signal_queue.push_back(FiredSignal {
            id,
            arguments: arguments.iter().map(|&v| v.clone()).collect(),
        });
    }

    pub fn queue_is_empty(&self) -> bool {
        self.signal_queue.is_empty()
    }

    pub fn queue_size(&self) -> usize {
        self.signal_queue.len()
    }

    pub fn get_meta(&self, id: u64) -> Option<&JsSignalMeta> {
        self.signal_registry.get(&id)
    }

    pub(super) fn pop_signal(&mut self) -> Option<FiredSignal> {
        self.signal_queue.pop_front()
    }
}

#[derive(Clone, Default, Debug)]
pub struct ManagerCtxRef(Rc<RefCell<JsManagerContext>>);

impl ManagerCtxRef {
    pub fn new(ctx: JsManagerContext) -> Self {
        Self(Rc::new(RefCell::new(ctx)))
    }

    pub fn downgrade(&self) -> ManagerCtxWeak {
        ManagerCtxWeak(Rc::downgrade(&self.0))
    }

    pub fn process_queue<'js>(&self, ctx: &Ctx<'js>) {
        let mut create_proxy = |ctx2: &Ctx<'js>, obj: Gd<godot::prelude::Object>| {
            let deps = ProxyDeps {
                ctx: ctx2.clone(),
                node: obj,
                manager_ctx: self.clone(),
            };
            create_godot_js_proxy(&deps)
        };

        loop {
            let mut this = self.0.borrow_mut();
            let f = this
                .pop_signal()
                .and_then(|s| this.get_meta(s.id).zip(Some(s.arguments)))
                .and_then(|(m, a)| Some((m.id, a, m.callback.clone().restore(ctx).ok()?)));
            drop(this);

            if let Some((id, args, f)) = f {
                let mut js_args: Vec<Value<'js>> = Vec::with_capacity(args.len());
                for arg in args {
                    match godot_variant_to_js(ctx, arg, &mut create_proxy) {
                        Ok(v) => js_args.push(v),
                        Err(_) => js_args.push(Value::new_undefined(ctx.clone())),
                    }
                }
                let res: rquickjs::Result<()> = f.call((Rest(js_args),)).map(|_: Value| ());
                if let Err(e) = res {
                    godot_error!("Error inside JS Signal Callback (ID {}): {:?}", id, e);
                }
            } else {
                break;
            }
        }
    }
}

impl Deref for ManagerCtxRef {
    type Target = RefCell<JsManagerContext>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Default, Debug)]
pub struct ManagerCtxWeak(Weak<RefCell<JsManagerContext>>);

impl ManagerCtxWeak {
    pub fn upgrade(&self) -> Option<ManagerCtxRef> {
        self.0.upgrade().map(ManagerCtxRef)
    }
}
