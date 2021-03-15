use crate::runtime::*;
use crate::*;
use std::*;

pub mod expression;
pub mod statement;

pub use expression::*;
pub use statement::*;

#[derive(Debug)]
pub struct ExecutionError {}

pub type Result<T> = core::result::Result<T, ExecutionError>;

#[allow(clippy::upper_case_acronyms)]
pub trait ASTNode: fmt::Debug {
    fn eval(&mut self, context: &mut Context) -> Result<Value>;
}

#[derive(Debug, Default)]
pub struct Program {
    statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Program {
        Program {
            statements: Vec::new(),
        }
    }

    pub fn statements_mut(&mut self) -> &mut Vec<Statement> {
        &mut self.statements
    }

    pub fn statements(&self) -> &Vec<Statement> {
        &self.statements
    }

    pub fn add_statement(&mut self, statement: Statement) {
        self.statements.push(statement);
    }
}
