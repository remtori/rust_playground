#![feature(assoc_char_funcs)]

use js::{
    ast::{ASTNode, Expression},
    parser::{lexer::Lexer, token::TokenKind, Parser},
    runtime::Value,
    Context,
};

fn main() {
    let mut context = Context::new();

    let mut parser = Parser::new(
        r#"
            var a = 5;
            a
            a = 4;
            a += 5
            a -= 2
            a *= 3
            a /= 2
            a ^= 1
            a %= 2
        "#,
    );

    match parser.parse_program() {
        Ok(mut program) => {
            for statement in program.statements_mut().iter_mut() {
                println!("{:?}", statement.eval(&mut context));
            }
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}
