use super::*;

#[derive(Debug)]
pub enum Statement {
    ExpressionStatement(Expression),
    VariableDeclaration(VariableDeclaration),
}

impl ASTNode for Statement {
    fn eval(&mut self, context: &mut Context) -> Value {
        match self {
            Statement::ExpressionStatement(expr) => expr.eval(context),
            _ => todo!()
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum DeclarationKind {
    Const,
    Let,
    Var,
}

#[derive(Debug)]
pub struct VariableDeclarator {
    identifier: Identifier,
    expression: Expression,
}

#[derive(Debug)]
pub struct VariableDeclaration {
    kind: DeclarationKind,
    declarations: Vec<VariableDeclarator>,
}