#![feature(
    assoc_char_funcs,
    default_free_fn
)]

use std::collections::HashMap;

pub mod ast;
pub mod js;
pub mod parser;
pub mod builtin;

#[macro_use]
extern crate lazy_static;

#[derive(Debug, Default)]
pub struct Context {
    variables: HashMap<String, js::Value>,
}

impl Context {
    pub fn new() -> Context {
        Context {
            variables: HashMap::new(),
        }
    }

    pub fn get_variable(&self, name: &str) -> Option<js::Value> {
        self.variables.get(name).map(Clone::clone)
    }

    pub fn set_variable(&mut self, name: String, value: js::Value) {
        self.variables.insert(name, value);
    }
}
