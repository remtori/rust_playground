use std::{collections::{HashMap, HashSet}, io::Write};

use crate::error::{Error, Result};


#[derive(Debug, Default)]
pub struct Context<'s> {
    pub(crate) native_type: HashMap<&'static str, Box<dyn ProtobinType<'static>>>,
    pub(crate) declared_message: HashMap<&'s str, Message<'s>>,
    pub(crate) required_ident: HashSet<&'s str>,
}

impl<'s> Context<'s> {
    pub fn new() -> Context<'s> {
        Default::default()
    }

    pub fn register(&mut self, msg: Message<'s>) {
        let ident = msg.identifier();
        self.declared_message.insert(ident, msg);
    }

    pub fn validate(&mut self) -> Result<()> {
        {
            let required = &mut self.required_ident;
            let mut cb = |ident| {
                required.insert(ident);
            };

            for typ in self.declared_message.values() {
                // (*typ).validate(&mut cb);
            }
        }

        let mut required = Vec::new();

        let mut error_buffer_capacity = 128;
        for typ in self.required_ident.iter() {
            if !self.declared_message.contains_key(typ) {
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
}

#[derive(Debug)]
pub struct Message<'s> {
    pub(crate) identifier: TypeDeclaration<'s>,
    pub(crate) extends: Vec<&'s str>,
    pub(crate) fields: Vec<FieldDeclaration<'s>>,
}

#[derive(Debug)]
pub struct FieldDeclaration<'s> {
    pub(crate) id: u32,
    pub(crate) ident: &'s str,
    pub(crate) ty: TypeDeclaration<'s>,
}

#[derive(Debug)]
pub struct TypeDeclaration<'s> {
    pub(crate) ident: &'s str,
    pub(crate) generics: Vec<&'s str>,
}


pub trait ProtobinType<'s> : std::fmt::Debug {
    fn identifier(&self) -> &'s str;

    fn write_serializer(&self, writer: &mut Writer, ty: &TypeDeclaration);

    fn write_deserializer(&self, writer: &mut Writer, ty: &TypeDeclaration);

    fn write_field_serializer(&self, writer: &mut Writer, field: &FieldDeclaration);

    fn write_field_derializer(&self, writer: &mut Writer, field: &FieldDeclaration);
}

impl<'s> ProtobinType<'s> for Message<'s> {
    fn identifier(&self) -> &'s str {
        self.identifier.ident
    }

    fn write_field_serializer(&self, writer: &mut Writer, field: &FieldDeclaration) {
        write!(writer, "a");
    }

    fn write_field_derializer(&self, writer: &mut Writer, field: &FieldDeclaration) {
        todo!()
    }

    fn write_serializer(&self, writer: &mut Writer, ty: &TypeDeclaration) {
        todo!()
    }

    fn write_deserializer(&self, writer: &mut Writer, ty: &TypeDeclaration) {
        todo!()
    }
}

pub struct Writer{}

impl std::io::Write for Writer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        todo!()
    }

    fn flush(&mut self) -> std::io::Result<()> {
        todo!()
    }
}
