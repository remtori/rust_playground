use crate::js::*;
use crate::*;
use std::*;

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

    pub fn statements(&self) -> &Vec<Statement> {
        &self.statements
    }

    pub fn add_statement(&mut self, statement: Statement) {
        self.statements.push(statement);
    }
}

#[derive(Debug)]
pub enum Statement {
    ExpressionStatement(Expression),
}

impl ASTNode for Statement {
    fn eval(&mut self, context: &mut Context) -> Value {
        match self {
            Statement::ExpressionStatement(expr) => expr.eval(context),
        }
    }
}

#[derive(Debug)]
pub enum ExpressionKind {
    Invalid,

    Identifier(String),

    // Literal
    Null,
    Boolean(bool),
    Numeric(f64),
    String(String),

    // Binary Operators
    Add(Box<dyn ASTNode>, Box<dyn ASTNode>),
    Sub(Box<dyn ASTNode>, Box<dyn ASTNode>),
    Mult(Box<dyn ASTNode>, Box<dyn ASTNode>),
}

#[derive(Debug)]
pub struct Expression {
    kind: ExpressionKind,
}

impl ASTNode for Expression {
    fn eval(&mut self, context: &mut Context) -> Value {
        match &mut self.kind {
            ExpressionKind::Invalid => unreachable!(),

            ExpressionKind::Null => Value::Null,
            ExpressionKind::Boolean(b) => Value::Boolean(*b),
            ExpressionKind::Numeric(v) => Value::Number(*v),
            ExpressionKind::String(s) => Value::String(s.clone()),

            ExpressionKind::Add(lhs, rhs)
            | ExpressionKind::Sub(lhs, rhs)
            | ExpressionKind::Mult(lhs, rhs) => {
                let left_value = lhs.eval(context);
                let right_value = rhs.eval(context);
                if let Value::Number(left_number) = left_value.to_number() {
                    if let Value::Number(right_number) = right_value.to_number() {
                        return match self.kind {
                            ExpressionKind::Add(_, _) => Value::Number(left_number + right_number),
                            ExpressionKind::Sub(_, _) => Value::Number(left_number - right_number),
                            ExpressionKind::Mult(_, _) => Value::Number(left_number * right_number),
                            _ => unreachable!(),
                        };
                    }
                }

                if let ExpressionKind::Add(_, _) = self.kind {
                    let mut res = left_value.to_string();
                    res.push_str(&right_value.to_string());
                    return Value::String(res);
                }

                Value::Number(f64::NAN)
            }
            _ => todo!(),
        }
    }
}