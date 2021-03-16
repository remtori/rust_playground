use super::*;

#[derive(Debug)]
pub enum Statement {
    ExpressionStatement(Expression),
    VariableDeclaration(VariableDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    ReturnStatement(Expression),
}

impl ASTNode for Statement {
    fn eval(&mut self, context: &mut Context) -> Result<Value> {
        match self {
            Statement::ExpressionStatement(expr) => expr.eval(context),
            Statement::VariableDeclaration(vd) => vd.eval(context),
            Statement::FunctionDeclaration(fd) => fd.eval(context),
            Statement::ReturnStatement(expr) => expr.eval(context),
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

#[derive(Debug, Default)]
pub struct BlockStatement {
    statements: Vec<Statement>,
}

impl BlockStatement {
    pub fn new() -> BlockStatement {
        BlockStatement {
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

impl ASTNode for BlockStatement {
    fn eval(&mut self, context: &mut Context) -> Result<Value> {
        for statement in self.statements.iter_mut() {
            statement.eval(context)?;
        }

        Ok(Value::Undefined)
    }
}

#[derive(Debug)]
pub struct FunctionDeclaration {
    ident: Identifier,
    params: Vec<Identifier>,
    body: BlockStatement,
}

impl ASTNode for FunctionDeclaration {
    fn eval(&mut self, context: &mut Context) -> Result<Value> {
        context.set_variable(self.ident.name().clone(), todo!());
        Ok(Value::Undefined)
    }
}

impl FunctionDeclaration {
    pub fn new(
        ident: Identifier,
        params: Vec<Identifier>,
        body: BlockStatement,
    ) -> FunctionDeclaration {
        FunctionDeclaration {
            ident,
            params,
            body,
        }
    }
}
