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

