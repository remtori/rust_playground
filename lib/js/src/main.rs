#![feature(assoc_char_funcs)]

use js::{
    ast::{ASTNode, Expression},
    value::Value,
    parser::{lexer::Lexer, token::TokenKind, Parser},
    Context
};

fn main() {
    let mut context = Context::new();

    let mut parser = Parser::new(
        r#"
            var a = 5;
            a
            a = 4;
            a
        "#,
    );

    let mut program = parser.parse_program();
    for statement in program.statements_mut().iter_mut() {
        println!("{:?}", statement.eval(&mut context));
    }
}
