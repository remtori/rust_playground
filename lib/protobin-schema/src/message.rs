use std::sync::atomic::{AtomicU32, Ordering};

use proc_macro2::Span;
use syn::{Error, Fields};

#[derive(Debug)]
pub enum Type {
    Ident(String),
    I32,
    I64,
    U32,
    U64,
    String,
    Bool,
    Bytes,
}

#[derive(Debug)]
pub struct Field {
    idx: u32,
    ident: String,
    ty: Type,
}

#[derive(Debug)]
pub struct Message {
    id: u32,
    name: String,
    fields: Vec<Field>,
}

static ID: AtomicU32 = AtomicU32::new(1);

fn new_id() -> u32 {
    ID.fetch_add(1, Ordering::Relaxed)
}

impl Message {
    pub fn parse(input: &syn::ItemStruct) -> syn::Result<Self> {
        let mut out_fields = Vec::new();

        match &input.fields {
            Fields::Named(fields) => {
                for (idx, field) in fields.named.iter().enumerate() {
                    out_fields.push(Field {
                        idx: (idx + 1) as u32,
                        ident: field.ident.as_ref().unwrap().to_string(),
                        ty: Type::Bool,
                    });
                }
            }
            Fields::Unnamed(_) | Fields::Unit => {
                return Err(Error::new(
                    Span::call_site(),
                    "expected struct with named field",
                ))
            }
        }

        Ok(Message {
            id: new_id(),
            name: input.ident.to_string(),
            fields: out_fields,
        })
    }
}
