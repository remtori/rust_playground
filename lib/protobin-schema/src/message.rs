use std::sync::atomic::{AtomicU32, Ordering};

use proc_macro2::Span;
use quote::ToTokens;
use syn::{Error, Fields};

#[derive(Debug)]
pub enum Type {
    Ident(String),
    IdentWithGenerics(String, Vec<Type>),
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

impl Field {
    pub fn index(&self) -> u32 {
        self.idx
    }

    pub fn name(&self) -> &str {
        &self.ident
    }

    pub fn ty(&self) -> &Type {
        &self.ty
    }
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
    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn fields(&self) -> &[Field] {
        &self.fields
    }

    pub fn parse(input: &syn::ItemStruct) -> syn::Result<Self> {
        let mut out_fields = Vec::new();

        match &input.fields {
            Fields::Unnamed(_) | Fields::Unit => {
                return Err(Error::new(
                    Span::call_site(),
                    "expected struct with named field",
                ))
            }
            Fields::Named(fields) => {
                for (idx, field) in fields.named.iter().enumerate() {
                    out_fields.push(Field {
                        idx: (idx + 1) as u32,
                        ident: field.ident.as_ref().unwrap().to_string(),
                        ty: resolve_type(&field.ty),
                    });
                }
            }
        }

        Ok(Message {
            id: new_id(),
            name: input.ident.to_string(),
            fields: out_fields,
        })
    }
}

fn resolve_type(ty: &syn::Type) -> Type {
    match ty {
        syn::Type::Path(p) => match p.path.get_ident() {
            Some(ident) => {
                let ident = ident.to_string();
                match ident.as_str() {
                    "i8" | "i16" | "i32" => Type::I32,
                    "i64" => Type::I64,
                    "u8" | "u16" | "u32" => Type::U32,
                    "u64" => Type::U64,
                    "bool" => Type::Bool,
                    "String" => Type::String,
                    _ => resolve_complex_type(ty),
                }
            }
            None => resolve_complex_type(ty),
        },
        _ => {
            unimplemented!()
        }
    }
}

fn resolve_complex_type(ty: &syn::Type) -> Type {
    if let syn::Type::Path(path) = ty {
        if let Some(seg) = path.path.segments.last() {
            match &seg.arguments {
                syn::PathArguments::None => Type::Ident(seg.ident.to_string()),
                syn::PathArguments::AngleBracketed(generics) => Type::IdentWithGenerics(
                    seg.ident.to_string(),
                    generics
                        .args
                        .iter()
                        .filter_map(|gt| {
                            // Ignore other generic like lifetime
                            if let syn::GenericArgument::Type(ty) = gt {
                                Some(resolve_type(ty))
                            } else {
                                None
                            }
                        })
                        .collect(),
                ),
                syn::PathArguments::Parenthesized(_) => unimplemented!(),
            }
        } else {
            unreachable!()
        }
    } else {
        println!("Can not resolve type: {}", ty.to_token_stream().to_string());
        unimplemented!()
    }
}
