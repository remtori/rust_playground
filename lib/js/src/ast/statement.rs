use super::*;

#[derive(Debug)]
pub enum Statement {
    ExpressionStatement(Expression),
    VariableDeclaration(VariableDeclaration),
}

impl ASTNode for Statement {
    fn eval(&mut self, context: &mut Context) -> Result<Value> {
        match self {
            Statement::ExpressionStatement(expr) => expr.eval(context),
            Statement::VariableDeclaration(vd) => vd.eval(context),
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
pub struct VariableDeclaration {
    pub(crate) kind: DeclarationKind,
    pub(crate) declarations: Vec<(Identifier, Expression)>,
}

impl ASTNode for VariableDeclaration {
    fn eval(&mut self, context: &mut Context) -> Result<Value> {
        for (id, init) in self.declarations.iter_mut() {
            let init_value = init.eval(context)?;
            context.set_variable(id.name().clone(), init_value);
        }

        Ok(Value::Undefined)
    }
}

impl VariableDeclaration {
    pub fn new(kind: DeclarationKind) -> VariableDeclaration {
        VariableDeclaration {
            kind,
            declarations: Vec::new(),
        }
    }

    pub fn add(&mut self, identifier: Identifier, initializer: Expression) {
        self.declarations.push((identifier, initializer));
    }
}
