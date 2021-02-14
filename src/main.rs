use rust_js::{
    ast::{Expression, Operator},
    js::Value,
    parser::{lexer::Lexer, token::TokenType},
    Context,
};

fn main() {
    let mut context = Context::new();

    let mut expr = Operator::Add {
        lhs: Box::new(Operator::Mult {
            lhs: Box::new(Value::Number(4.0)),
            rhs: Box::new(Value::Number(3.0)),
        }),
        rhs: Box::new(Operator::Sub {
            lhs: Box::new(Value::Number(2.0)),
            rhs: Box::new(Value::Number(1.0)),
        }),
    };

    let value = expr.eval(&mut context);

    println!("Exec value: {:?}", value);

    let mut lexer = Lexer::new(
        r#"
            4 + (6 - 9) * 2;
            "hello, world"
            '!'
            "/?"
        "#,
    );

    loop {
        let token = lexer.next_token();
        println!("{:?}", token);

        if token.token_type() == TokenType::Eof {
            break;
        }
    }
}
