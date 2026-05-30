use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::ops::Deref;
use std::rc::{Rc, Weak};

use rquickjs::{Ctx, Function, Persistent};

#[derive(Default, Debug)]
pub struct JsManagerContext {
    last_id: u64,
    signal_registry: HashMap<u64, Persistent<Function<'static>>>,
    signal_queue: VecDeque<u64>,
}

impl JsManagerContext {
    pub fn clear(&mut self) {
        self.last_id = 0;
        self.signal_registry.clear();
        self.signal_queue.clear();
    }

    fn next_callback_id(&mut self) -> u64 {
        self.last_id += 1;
        self.last_id
    }

    pub fn save_callback<'js>(&mut self, ctx: &Ctx<'js>, f: Function<'js>) -> u64 {
        let id = self.next_callback_id();
        self.signal_registry.insert(id, Persistent::save(ctx, f));
        id
    }

    pub fn remove_callback(&mut self, id: u64) {
        self.signal_registry.remove(&id);
    }

    pub fn enqueue(&mut self, id: u64) {
        self.signal_queue.push_back(id);
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
