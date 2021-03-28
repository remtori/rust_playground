use std::str::FromStr;

fn main() {
    let source = r#"
        struct Ping {
            unix_time: u32,
        }
    "#;

    let tokens = proc_macro2::TokenStream::from_str(&source).unwrap();
    let output = protobin_schema::derive_message(tokens);
    println!("{}", output);
}
