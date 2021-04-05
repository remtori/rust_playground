use std::collections::HashMap;

use crate::{
    error::{Error, Result},
    writer::Writer,
};

#[derive(Debug, Default)]
pub struct Context<'s> {
    pub(crate) native_type: HashMap<&'static str, Box<dyn ProtobinType<'static>>>,
    pub(crate) declared_message: HashMap<&'s str, Message<'s>>,
}

impl<'s> Context<'s> {
    pub fn register(&mut self, msg: Message<'s>) {
        let ident = msg.ident();
        self.declared_message.insert(ident, msg);
    }

    pub fn validate(&mut self) -> Result<()> {
        for msg in self.declared_message.values() {
            for field_ty in &msg.fields {
                let ty = field_ty.ty.ident;
                if !self.native_type.contains_key(ty) && !self.declared_message.contains_key(ty) {
                    return Err(Error::Message(
                        format!("Use of undeclared type: {} in {}:{}", ty, msg.ident(), field_ty.ident)
                    ));
                }
            }
        }        

        Ok(())        
    }
}

#[derive(Debug)]
pub struct Message<'s> {
    pub(crate) ident: &'s str,
    pub(crate) generics: Vec<&'s str>,
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
    fn ident(&self) -> &'s str;

    fn write_serializer(&self, writer: &mut Writer, ty: &TypeDeclaration);

    fn write_deserializer(&self, writer: &mut Writer, ty: &TypeDeclaration);

    fn write_field_serializer(&self, writer: &mut Writer, field: &FieldDeclaration);

    fn write_field_derializer(&self, writer: &mut Writer, field: &FieldDeclaration);
}
