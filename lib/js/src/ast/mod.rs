use crate::value::*;
use crate::*;
use std::*;

pub mod expression;
pub mod statement;

pub use expression::*;
pub use statement::*;

pub trait DebugPrint {
    fn debug_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result;
}

#[allow(clippy::upper_case_acronyms)]
pub trait ASTNode: DebugPrint {
    fn eval(&mut self, context: &mut Context) -> Value;
}

impl fmt::Debug for Box<dyn ASTNode> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.debug_fmt(f)
    }
}

impl<T: fmt::Debug + ASTNode> DebugPrint for T {
    fn debug_fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
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
