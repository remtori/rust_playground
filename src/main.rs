use rust_js::{
    ast::{Expression, Operator},
    js::Value,
    parser::{lexer::Lexer, token::TokenKind},
    Context,
};

fn main() {
    let mut context = Context::new();

    // let mut expr = Operator::Add {
    //     lhs: Box::new(Operator::Mult {
    //         lhs: Box::new(Value::Number(4.0)),
    //         rhs: Box::new(Value::Number(3.0)),
    //     }),
    //     rhs: Box::new(Operator::Sub {
    //         lhs: Box::new(Value::Number(2.0)),
    //         rhs: Box::new(Value::Number(1.0)),
    //     }),
    // };

    // let value = expr.eval(&mut context);

    // println!("Exec value: {:?}", value);

    let mut lexer = Lexer::new(
        r#"
            4 + (6 - 9) * 2;
            "hello, world"
            '!'
            "/?"
            a + 4 - 9 * b / 2
        "#,
    );

    loop {
        let token = lexer.next_token();
        println!("{:?}", token);

        if token.kind() == TokenKind::Eof {
            break;
        }
    }
}
