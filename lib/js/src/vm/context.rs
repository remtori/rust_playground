use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use super::Runtime;
use crate::{
    ast::DeclarationKind,
    gc::{GcCell, GcPointer},
    jsrt::JsValue,
};

pub struct Context {
    runtime: Arc<Mutex<Runtime>>,
    variables: HashMap<String, GcPointer<JsValue>>,
}

impl Context {
    pub fn new(rt: Arc<Mutex<Runtime>>) -> Context {
        Context {
            runtime: rt,
            variables: HashMap::new(),
        }
    }

    pub fn get_variable(&self, name: &str) -> JsValue {
        match self.variables.get(name) {
            Some(v) => v.clone_inner(),
            None => JsValue::undefined(),
        }
    }

    pub fn set_variable(&mut self, name: String, value: GcPointer<JsValue>) {
        self.variables.insert(name, value);
    }

    pub fn allocate<T>(&mut self, data: T) -> GcPointer<T>
    where
        T: GcCell,
    {
        self.runtime.lock().unwrap().heap().allocate(data)
    }
}

// #[derive(Debug)]
// pub struct Variable {
//     pub(crate) value: jsrt::Value,
//     pub(crate) declaration_kind: DeclarationKind,
// }

// impl Variable {
//     pub fn new(value: jsrt::Value, declaration_kind: DeclarationKind) -> Variable {
//         Variable {
//             value,
//             declaration_kind,
//         }
//     }

//     pub fn from_const(value: jsrt::Value) -> Variable {
//         Variable {
//             value,
//             declaration_kind: DeclarationKind::Const,
//         }
//     }

//     pub fn from_let(value: jsrt::Value) -> Variable {
//         Variable {
//             value,
//             declaration_kind: DeclarationKind::Let,
//         }
//     }
// }

// #[derive(Debug)]
// pub enum EnvironmentRecordKind {
//     Declarative,
//     Function,
//     Object,
//     Global,
//     Module,
// }

// #[derive(Debug)]
// enum ThisBindingStatus {
//     Lexical,
//     Initialized,
//     Uninitialized,
// }

// #[derive(Debug)]
// pub struct LexicalEnvironment {
//     kind: EnvironmentRecordKind,
//     variables: HashMap<String, Variable>,
//     this_status: ThisBindingStatus,
//     this_value: Option<jsrt::Value>,
//     super_value: Option<jsrt::Value>,
//     with_base_value: jsrt::Value,
//     parent: Option<Arc<LexicalEnvironment>>,
// }

// impl LexicalEnvironment {
//     pub fn has_binding(&self, name: &str) -> bool {
//         self.variables.contains_key(name)
//     }

//     pub fn create_mutable_binding(&mut self, name: &str, _delete: bool) {
//         assert!(!self.variables.contains_key(name));

//         self.variables
//             .insert(name.into(), Variable::from_let(jsrt::Value::Undefined));

//         // if delete is true, record that the newly created binding may be deleted by a subsequent DeleteBinding call.
//     }

//     pub fn create_immutable_binding(&mut self, name: &str, _strict: bool) {
//         assert!(!self.variables.contains_key(name));

//         self.variables
//             .insert(name.into(), Variable::from_const(jsrt::Value::Undefined));
//     }

//     pub fn initialize_binding(&mut self, name: &str, value: jsrt::Value) {
//         assert!(self
//             .variables
//             .get(name)
//             .map(|v| matches!(&v.value, jsrt::Value::Undefined))
//             .is_some());

//         self.variables.get_mut(name).unwrap().value = value;
//     }

//     pub fn set_mutable_binding(&mut self, name: &str, value: jsrt::Value, strict: bool) {
//         todo!()
//     }

//     pub fn get_binding_value(&self, name: &str, strict: bool) -> Result<jsrt::Value, ()> {
//         todo!()
//     }

//     pub fn delete_binding(&mut self, name: &str) -> bool {
//         todo!()
//     }

//     pub fn has_this_binding(&self) -> bool {
//         self.this_value.is_some()
//     }

//     pub fn has_super_binding(&self) -> bool {
//         self.super_value.is_some()
//     }

//     pub fn with_base_object(&self) -> jsrt::Value {
//         // self.with_base_value
//         todo!()
//     }
// }
