use std::collections::{HashMap, HashSet};

use crate::error::{Error, Result};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Language {
    Rust,
    Typescript,
}

pub trait Node: std::fmt::Debug {
    fn is_builtin(&self) -> bool {
        false
    }

    fn identifier(&self) -> &str;

    fn serialize(
        &self,
        writer: &mut dyn std::io::Write,
        lang: Language,
    ) -> std::result::Result<(), std::io::Error>;

    fn validate(&self, require_ident: &mut dyn FnMut(String));
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

    pub fn validate(&mut self) -> Result<()> {
        {
            let required = &mut self.required_ident;
            let mut cb = |ident| {
                required.insert(ident);
            };

            for typ in self.declared_type.values() {
                (*typ).validate(&mut cb);
            }
        }

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
                msg.push_str("    ");
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
            (*node).serialize(writer, lang)?;
        }

        Ok(())
    }

    // pub fn get(&self, ident: &str) -> Option<&dyn Node> {
    //     self.declared_type.get(ident).map(|b| &**b)
    // }
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
        writer: &mut dyn std::io::Write,
        lang: Language,
    ) -> std::result::Result<(), std::io::Error> {
        match lang {
            Language::Rust => {
                writer.write_all(b"struct ")?;
                writer.write_all(self.identifier.as_bytes())?;
                if !self.extends.is_empty() {
                    writer.write_all(b": ")?;

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
                    (*value).serialize(writer, lang)?;
                    writer.write_all(b",\n")?;
                }
                writer.write_all(b"}\n")?;
            }
            Language::Typescript => {}
        }

        Ok(())
    }

    fn validate(&self, require: &mut dyn FnMut(String)) {
        for extend in &self.extends {
            require(extend.clone())
        }

        for typ in self.fields.values() {
            if !(*typ).is_builtin() {
                require((*typ).identifier().to_owned())
            }
        }
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
        writer: &mut dyn std::io::Write,
        _lang: Language,
    ) -> std::result::Result<(), std::io::Error> {
        writer.write_all(self.identifier.as_bytes())
    }

    fn validate(&self, require: &mut dyn FnMut(String)) {
        require(self.real_ident.clone());
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
        writer: &mut dyn std::io::Write,
        _lang: Language,
    ) -> std::result::Result<(), std::io::Error> {
        writer.write_all(self.identifier.as_bytes())
    }

    fn validate(&self, require: &mut dyn FnMut(String)) {
        require(self.identifier.clone())
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
    fn is_builtin(&self) -> bool {
        true
    }

    fn identifier(&self) -> &str {
        unreachable!()
    }

    fn serialize(
        &self,
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

    fn validate(&self, _: &mut dyn FnMut(String)) {}
}

#[derive(Debug)]
pub enum NativeNode {
    Option(Box<dyn Node>),
}

impl Node for NativeNode {
    fn is_builtin(&self) -> bool {
        true
    }

    fn identifier(&self) -> &str {
        unreachable!()
    }

    fn serialize(
        &self,
        writer: &mut dyn std::io::Write,
        lang: Language,
    ) -> std::result::Result<(), std::io::Error> {
        match (&self, lang) {
            (NativeNode::Option(node), Language::Rust) => {
                writer.write_all(b"Option<")?;
                (*node).serialize(writer, lang)?;
                writer.write_all(b">")?;
            }
            (NativeNode::Option(_), Language::Typescript) => {}
        }

        Ok(())
    }

    fn validate(&self, _: &mut dyn FnMut(String)) {}
}
