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

fn main() -> Result<()> {
    let source = r#"
        /// Send ping message to check if the other party is alive
        struct Ping {
            /// Sender timestamp
            unixTime: u32,
        }

        // comment break stuff
        struct Pong : Ping {}
    "#;

    let mut context = Context::new();

    let mut parser = Parser::new(source);
    parser.parse_schema(&mut context)?;

    println!("{:#?}", context);

    context.validate()?;

    context.export(Language::Rust, &mut std::io::stdout());

    Ok(())
}
