use crate::{
    context::{Context, Language},
    error::Result,
    parser::Parser,
};

mod context;
mod error;
mod lexer;
mod parser;
mod token;

fn exec() -> Result<()> {
    let source = r#"
        /// Send ping message to check if the other party is alive
        struct Ping {
            /// Sender timestamp
            unixTime: u32,
        }

        // comment break stuff
        struct Pong : Ping {}

        struct A: B{}
    "#;

    let mut context = Context::new();

    let mut parser = Parser::new(source);
    parser.parse_schema(&mut context)?;

    println!("{:#?}", context);

    context.validate()?;

    context.export(Language::Rust, &mut std::io::stdout())?;

    Ok(())
}

fn main() {
    if let Err(err) = exec() {
        println!("{}", err);
    }
}
