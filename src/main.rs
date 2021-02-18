#![feature(assoc_char_funcs)]

use rust_js::{
    ast::{ASTNode, Expression},
    js::Value,
    parser::{lexer::Lexer, token::TokenKind, Parser},
    Context,
    builtin::BigUInt,
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

    // let mut parser = Parser::new(
    //     r#"
    //         let a = 4
    //         const b = 4, c = 6, d = 7 * 9;
    //         var x = (9 - 4) * 2 + 5;
    //         var y = a - b * c;
    //         x
    //         y
    //         y + d
    //     "#,
    // );

    // let mut program = parser.parse_program();
    // for statement in program.statements_mut().iter_mut() {
    //     println!("{:?}", statement.eval(&mut context));
    // }

    let bi =  BigUInt::from(1234567890123);
    // let bi =  BigInt::from(5);
    println!("bit_str={}", bi.to_bit_string());
    println!("str    ={}", bi);
}
