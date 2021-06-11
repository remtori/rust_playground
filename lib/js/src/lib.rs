#![allow(clippy::upper_case_acronyms)]
#![feature(default_free_fn)]
#![feature(ptr_metadata)]

use std::collections::HashMap;

pub mod ast;
pub mod gc;
pub mod parser;
pub mod runtime;
pub mod vm;

#[macro_use]
extern crate lazy_static;

pub(crate) use vm::context::*;
