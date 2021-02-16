use rust_js::{
    ast::{ASTNode, Expression},
    js::Value,
    parser::{lexer::Lexer, token::TokenKind, Parser},
    Context,
};

fn main() {
    let mut context = Context::new();

    // let mut expr = Expression::Add(
    //     Box::new(Expression::Mult(
    //         Box::new(Value::Number(4.0)),
    //         Box::new(Value::Number(3.0)),
    //     )),
    //     Box::new(Expression::Sub(
    //         Box::new(Value::Number(2.0)),
    //         Box::new(Value::Number(1.0)),
    //     )),
    // );

    // let value = expr.eval(&mut context);

    // println!("Expr: {:?}", expr);
    // println!("Exec value: {:?}", value);

    // let mut lexer = Lexer::new(
    //     r#"
    //         4 + (6 - 9) * 2;
    //         "hello, world"
    //         '!'
    //         "/?"
    //         a + 4 - 9 * b / 2
    //     "#,
    // );

    // loop {
    //     let token = lexer.next_token();
    //     println!("{:?}", token);

    //     if token.kind() == TokenKind::Eof {
    //         break;
    //     }
    // }

    let mut parser = Parser::new(
        r#"
            4 + 2 * 6 - 9
        "#,
    );

    let mut program = parser.parse_program();
    for statement in program.statements_mut().iter_mut() {
        println!("{:?}", statement.eval(&mut context));
    }
}
