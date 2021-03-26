use std::collections::{HashMap, HashSet};

use crate::error::{Error, Result};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Language {
    Rust,
    Typescript,
}

pub trait Node: std::fmt::Debug {
    fn identifier(&self) -> &str;

    fn serialize(
        &self,
        context: &Context,
        writer: &mut dyn std::io::Write,
        lang: Language,
    ) -> std::result::Result<(), std::io::Error>;
}

#[derive(Debug, Default)]
pub struct Context {
    declared_type: HashMap<String, Box<dyn Node>>,
    required_ident: HashSet<String>,
}

impl Context {
    pub fn new() -> Context {
        Default::default()
    }

    pub fn register<T: 'static + Node>(&mut self, node: T) {
        let boxed_node = Box::new(node);
        let ident = boxed_node.identifier();
        self.declared_type.insert(ident.to_owned(), boxed_node);
    }

    pub fn require(&mut self, ident: &str) {
        self.required_ident.insert(ident.to_owned());
    }

    pub fn validate(&self) -> Result<()> {
        let mut required = Vec::new();

        let mut error_buffer_capacity = 128;
        for typ in self.required_ident.iter() {
            if !self.declared_type.contains_key(typ) {
                required.push(typ);
                error_buffer_capacity += typ.len();
            }
        }

        if !required.is_empty() {
            let mut msg = String::with_capacity(error_buffer_capacity);

            msg.push_str("Use of undeclared type: [\n");
            for str in required {
                msg.push_str(str);
                msg.push_str(",\n");
            }
            msg.push(']');

            Err(Error::Message(msg))
        } else {
            Ok(())
        }
    }

    pub fn export<T: std::io::Write>(
        &self,
        lang: Language,
        writer: &mut T,
    ) -> std::result::Result<(), std::io::Error> {
        for (_, node) in self.declared_type.iter() {
            (*node).serialize(self, writer, lang)?;
        }

        Ok(())
    }

    pub fn get(&self, ident: &str) -> Option<&dyn Node> {
        self.declared_type.get(ident).map(|b| &**b)
    }
}

#[derive(Debug)]
pub struct StructNode {
    pub(crate) identifier: String,
    pub(crate) extends: Vec<String>,
    pub(crate) fields: HashMap<String, Box<dyn Node>>,
}

impl Node for StructNode {
    fn identifier(&self) -> &str {
        &self.identifier
    }

    fn serialize(
        &self,
        context: &Context,
        writer: &mut dyn std::io::Write,
        lang: Language,
    ) -> std::result::Result<(), std::io::Error> {
        match lang {
            Language::Rust => {
                writer.write_all(b"struct ")?;
                writer.write_all(self.identifier.as_bytes())?;
                if !self.extends.is_empty() {
                    let mut first = true;
                    for extend in &self.extends {
                        if !first {
                            writer.write_all(b" + ")?;
                        }

                        writer.write_all(extend.as_bytes())?;
                        first = false;
                    }
                }

                writer.write_all(b" {\n")?;
                for (field, value) in &self.fields {
                    writer.write_all(b"\t")?;
                    writer.write_all(field.as_bytes())?;
                    writer.write_all(b": ")?;
                    (*value).serialize(context, writer, lang)?;
                    writer.write_all(b",\n")?;
                }
                writer.write_all(b"}\n")?;
            }
            Language::Typescript => {}
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct AliasNode {
    pub(crate) identifier: String,
    pub(crate) real_ident: String,
}

impl Node for AliasNode {
    fn identifier(&self) -> &str {
        &self.identifier
    }

    fn serialize(
        &self,
        context: &Context,
        writer: &mut dyn std::io::Write,
        lang: Language,
    ) -> std::result::Result<(), std::io::Error> {
        context
            .get(&self.real_ident)
            .unwrap()
            .serialize(context, writer, lang)
    }
}

#[derive(Debug)]
pub struct IdentifierNode {
    pub(crate) identifier: String,
}

impl Node for IdentifierNode {
    fn identifier(&self) -> &str {
        &self.identifier
    }

    fn serialize(
        &self,
        context: &Context,
        writer: &mut dyn std::io::Write,
        lang: Language,
    ) -> std::result::Result<(), std::io::Error> {
        context
            .get(&self.identifier)
            .unwrap()
            .serialize(context, writer, lang)
    }
}

#[derive(Debug)]
pub enum PrimitiveNode {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
    String,
    Char,

    /// equivalent to Rust "()" type
    Unit,
}

impl Node for PrimitiveNode {
    fn identifier(&self) -> &str {
        unreachable!()
    }

    fn serialize(
        &self,
        _context: &Context,
        writer: &mut dyn std::io::Write,
        lang: Language,
    ) -> std::result::Result<(), std::io::Error> {
        match (self, lang) {
            (PrimitiveNode::I8, Language::Rust) => writer.write_all(b"i8")?,
            (PrimitiveNode::I16, Language::Rust) => writer.write_all(b"i16")?,
            (PrimitiveNode::I32, Language::Rust) => writer.write_all(b"i32")?,
            (PrimitiveNode::I64, Language::Rust) => writer.write_all(b"i64")?,
            (PrimitiveNode::U8, Language::Rust) => writer.write_all(b"u8")?,
            (PrimitiveNode::U16, Language::Rust) => writer.write_all(b"u16")?,
            (PrimitiveNode::U32, Language::Rust) => writer.write_all(b"u32")?,
            (PrimitiveNode::U64, Language::Rust) => writer.write_all(b"u64")?,
            (PrimitiveNode::F32, Language::Rust) => writer.write_all(b"f32")?,
            (PrimitiveNode::F64, Language::Rust) => writer.write_all(b"f64")?,
            (PrimitiveNode::String, Language::Rust) => writer.write_all(b"String")?,
            (PrimitiveNode::Char, Language::Rust) => writer.write_all(b"char")?,
            (PrimitiveNode::Unit, Language::Rust) => writer.write_all(b"()")?,

            (PrimitiveNode::I8, Language::Typescript)
            | (PrimitiveNode::I16, Language::Typescript)
            | (PrimitiveNode::I32, Language::Typescript)
            | (PrimitiveNode::I64, Language::Typescript)
            | (PrimitiveNode::U8, Language::Typescript)
            | (PrimitiveNode::U16, Language::Typescript)
            | (PrimitiveNode::U32, Language::Typescript)
            | (PrimitiveNode::U64, Language::Typescript)
            | (PrimitiveNode::F32, Language::Typescript)
            | (PrimitiveNode::F64, Language::Typescript) => writer.write_all(b"number")?,

            (PrimitiveNode::String, Language::Typescript)
            | (PrimitiveNode::Char, Language::Typescript) => writer.write_all(b"string")?,

            (PrimitiveNode::Unit, Language::Typescript) => {}
        }

        Ok(())
    }
}

#[derive(Debug)]
pub enum NativeNode {
    Option(Box<dyn Node>),
}

impl Node for NativeNode {
    fn identifier(&self) -> &str {
        unreachable!()
    }

    fn serialize(
        &self,
        context: &Context,
        writer: &mut dyn std::io::Write,
        lang: Language,
    ) -> std::result::Result<(), std::io::Error> {
        match (&self, lang) {
            (NativeNode::Option(node), Language::Rust) => {
                writer.write_all(b"Option<")?;
                (*node).serialize(context, writer, lang)?;
                writer.write_all(b">")?;
            }
            (NativeNode::Option(_), Language::Typescript) => {}
        }

        Ok(())
    }
}
