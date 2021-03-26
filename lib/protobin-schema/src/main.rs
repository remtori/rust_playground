mod error;
mod lexer;
mod token;

fn main() {
    let source = r#"
        message Ping {
            unixTime: u32,
        }

        message Pong : Ping {}
    "#;

    let mut lexer = lexer::Lexer::new(source);

    loop {
        match lexer.next_token() {
            Ok(token) => {
                println!("{:?}", token);
            }
            Err(err) => {
                println!("{}", err);
                break;
            }
        }
    }
}
