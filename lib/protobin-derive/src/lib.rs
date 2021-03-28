#[proc_macro_derive(Message)]
pub fn derive_message(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = protobin_schema::proc_macro2::TokenStream::from(input);
    let output = protobin_schema::derive_message(input);
    proc_macro::TokenStream::from(output)
}
