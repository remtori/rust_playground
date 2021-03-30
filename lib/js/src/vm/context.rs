use std::sync::Arc;

use crate::*;
use crate::{ast::DeclarationKind, runtime::value};

#[derive(Debug, Default)]
pub struct Context {
    variables: HashMap<String, runtime::Value>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            variables: HashMap::new(),
        }
    }

    pub fn get_variable(&self, name: &str) -> Option<runtime::Value> {
        self.variables.get(name).map(Clone::clone)
    }

    pub fn set_variable(&mut self, name: String, value: runtime::Value) {
        self.variables.insert(name, value);
    }
}

#[derive(Debug)]
pub struct Variable {
    pub(crate) value: runtime::Value,
    pub(crate) declaration_kind: DeclarationKind,
}

impl Variable {
    pub fn new(value: runtime::Value, declaration_kind: DeclarationKind) -> Variable {
        Variable {
            value,
            declaration_kind,
        }
    }

    pub fn from_const(value: runtime::Value) -> Variable {
        Variable {
            value,
            declaration_kind: DeclarationKind::Const,
        }
    }

    pub fn from_let(value: runtime::Value) -> Variable {
        Variable {
            value,
            declaration_kind: DeclarationKind::Let,
        }
    }
}

#[derive(Debug)]
pub enum EnvironmentRecordKind {
    Declarative,
    Function,
    Object,
    Global,
    Module,
}

#[derive(Debug)]
enum ThisBindingStatus {
    Lexical,
    Initialized,
    Uninitialized,
}

#[derive(Debug)]
pub struct LexicalEnvironment {
    kind: EnvironmentRecordKind,
    variables: HashMap<String, Variable>,
    this_status: ThisBindingStatus,
    this_value: Option<runtime::Value>,
    super_value: Option<runtime::Value>,
    with_base_value: runtime::Value,
    parent: Option<Arc<LexicalEnvironment>>,
}

impl LexicalEnvironment {
    pub fn has_binding(&self, name: &str) -> bool {
        self.variables.contains_key(name)
    }

    pub fn create_mutable_binding(&mut self, name: &str, _delete: bool) {
        assert!(!self.variables.contains_key(name));

        self.variables
            .insert(name.into(), Variable::from_let(runtime::Value::Undefined));

        // if delete is true, record that the newly created binding may be deleted by a subsequent DeleteBinding call.
    }

    pub fn create_immutable_binding(&mut self, name: &str, _strict: bool) {
        assert!(!self.variables.contains_key(name));

        self.variables
            .insert(name.into(), Variable::from_const(runtime::Value::Undefined));
    }

    pub fn initialize_binding(&mut self, name: &str, value: runtime::Value) {
        assert!(self
            .variables
            .get(name)
            .map(|v| matches!(&v.value, runtime::Value::Undefined))
            .is_some());

        self.variables.get_mut(name).unwrap().value = value;
    }

    pub fn set_mutable_binding(&mut self, name: &str, value: runtime::Value, strict: bool) {
        todo!()
    }

    pub fn get_binding_value(&self, name: &str, strict: bool) -> Result<runtime::Value, ()> {
        todo!()
    }

    pub fn delete_binding(&mut self, name: &str) -> bool {
        todo!()
    }

    pub fn has_this_binding(&self) -> bool {
        self.this_value.is_some()
    }

    pub fn has_super_binding(&self) -> bool {
        self.super_value.is_some()
    }

    pub fn with_base_object(&self) -> runtime::Value {
        // self.with_base_value
        todo!()
    }
}
