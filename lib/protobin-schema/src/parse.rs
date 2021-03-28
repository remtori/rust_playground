use quote::ToTokens;
use syn::{parenthesized, parse::Parse, parse2, punctuated::Punctuated, Item, Result, Token, Type};

use crate::message::Message;

pub fn parse(source: &str) -> Result<Vec<Message>> {
    let mut out = Vec::new();

    // Parse the whole file
    let file = syn::parse_file(source)?;

    // Iterate over all the items
    for item in file.items.iter() {
        // If we found a struct
        if let Item::Struct(struct_data) = item {
            // Check the attributes #[derive(Message)]
            for attr in struct_data.attrs.iter() {
                if_chain::if_chain! {
                    if let Some(ident) = attr.path.get_ident();
                    if ident.eq("derive");
                    then {
                        for ident in parse2::<DeriveAttr>(attr.tokens.to_token_stream())?.0 {
                            if_chain::if_chain! {
                                if let Type::Path(p) = ident;
                                if let Some(maybe_msg) = p.path.get_ident();
                                if maybe_msg.eq("Message");
                                then {
                                    out.push(Message::parse(struct_data)?);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(out)
}

struct DeriveAttr(Punctuated<Type, Token![,]>);

impl Parse for DeriveAttr {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let content;
        let _ = parenthesized!(content in input);

        Ok(Self(content.parse_terminated(Type::parse)?))
    }
}
