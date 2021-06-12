use std::{collections::HashMap, ops::Deref};

use super::{JsString, JsValue};
use crate::{gc::*, vm::Context};

#[derive(Debug, GcTrace)]
pub struct JsObject {
    properties: HashMap<u64, GcPointer<JsValue>>,
}

impl JsObject {
    pub fn new(ctx: &mut Context) -> GcPointer<JsObject> {
        ctx.allocate(JsObject {
            properties: HashMap::new(),
        })
    }

    pub fn put_property(&mut self, key: GcPointer<JsString>, value: GcPointer<JsValue>) {
        self.properties.insert(key.string.hash(), value);
    }

    pub fn get_property(&self, key: GcPointer<JsString>) -> JsValue {
        match self.properties.get(&key.string.hash()) {
            Some(v) => v.deref().clone(),
            None => JsValue::undefined(),
        }
    }
}

impl GcCell for JsObject {}
