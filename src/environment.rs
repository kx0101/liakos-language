use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::object::Object;

pub type EnvRef = Rc<RefCell<Environment>>;

#[derive(Debug, Default)]
pub struct Environment {
    store: HashMap<String, Object>,
    outer: Option<EnvRef>,
}

impl Environment {
    pub fn new() -> EnvRef {
        Rc::new(RefCell::new(Self::default()))
    }

    pub fn new_enclosed(outer: EnvRef) -> EnvRef {
        Rc::new(RefCell::new(Self {
            store: HashMap::new(),
            outer: Some(outer),
        }))
    }

    pub fn get(&self, name: &str) -> Option<Object> {
        if let Some(v) = self.store.get(name) {
            return Some(v.clone());
        }
        if let Some(outer) = &self.outer {
            return outer.borrow().get(name);
        }
        None
    }

    pub fn set(&mut self, name: impl Into<String>, val: Object) -> Object {
        let v = val.clone();
        self.store.insert(name.into(), val);
        v
    }
}
