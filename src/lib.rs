pub mod ast;
pub mod js;
pub mod parser;

#[macro_use]
extern crate lazy_static;

#[derive(Debug, Default)]
pub struct Context;

impl Context {
    pub fn new() -> Context {
        Context {}
    }
}
