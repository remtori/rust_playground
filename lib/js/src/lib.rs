#![allow(clippy::upper_case_acronyms)]
#![feature(default_free_fn)]
#![feature(ptr_metadata)]

pub mod ast;
pub mod gc;
pub mod jsrt;
pub mod parser;
pub mod vm;

#[macro_use]
extern crate lazy_static;

pub(crate) use vm::context::*;
