use syn::{
    parenthesized, parse::Parse, punctuated::Punctuated, Item, Meta, NestedMeta, Path, Result,
    Token, Type,
};

use crate::message::Message;

pub fn parse(source: &str, out: &mut Vec<Message>) -> Result<()> {
    // Parse the whole file
    let file = syn::parse_file(source)?;

    // Iterate over all the items
    for item in file.items.iter() {
        // If we found a struct
        if let Item::Struct(struct_data) = item {
            // Check the attributes #[derive(Message)]
            for attr in struct_data.attrs.iter() {
                let meta = attr.parse_meta().unwrap();
                if meta.path().is_ident("derive") {
                    if let Meta::List(meta_list) = &meta {
                        if meta_list.nested.iter().any(|c| {
                            if let NestedMeta::Meta(meta) = c {
                                meta.path().is_ident("Message")
                            } else {
                                false
                            }
                        }) {
                            out.push(Message::parse(struct_data)?);
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

struct DeriveAttr(Punctuated<Type, Token![,]>);

impl Parse for DeriveAttr {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let content;
        let _ = parenthesized!(content in input);

        Ok(Self(content.parse_terminated(Type::parse)?))
    }
}
