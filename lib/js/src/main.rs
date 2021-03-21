#![feature(assoc_char_funcs)]

use js::{
    ast::{ASTNode, Expression},
    parser::{lexer::Lexer, token::TokenKind, Parser},
    runtime::Value,
    vm::Context,
};

fn main() {
    let mut context = Context::new();

    let mut parser = Parser::new(
        r#"
            function a() {
                return 4;
            }

            function b(a) {
            }

            function c(a, b, c) {
                let a = 4;

                b = 3;
                var c = 2;
                return a + b * c;
            }
        "#,
    );

    match parser.parse_program() {
        Ok(mut program) => {
            println!("{:#?}", program);

            for statement in program.statements_mut().iter_mut() {
                println!("{:?}", statement.eval(&mut context));
            }
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}
