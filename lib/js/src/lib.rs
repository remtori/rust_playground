#![allow(clippy::upper_case_acronyms)]
#![feature(assoc_char_funcs, default_free_fn)]

use std::collections::HashMap;

pub mod ast;
pub mod parser;
pub mod runtime;
pub mod vm;

#[macro_use]
extern crate lazy_static;

pub(crate) use vm::context::*;
