use std::collections::HashMap;

use crate::{
    context::{Context, IdentifierNode, NativeNode, Node, PrimitiveNode, StructNode},
    error::{Error, ParseError, Result},
    lexer::Lexer,
    token::{Token, TokenKind},
};

pub struct Parser<'s> {
    lexer: Lexer<'s>,
    current_token: Token<'s>,
}

impl<'s> Parser<'s> {
    pub fn new(source: &'s str) -> Parser<'s> {
        let mut lexer = Lexer::new(source);
        let current_token = lexer.next_token().unwrap();

        Parser {
            lexer,
            current_token,
        }
    }

    pub fn parse_schema(&mut self, context: &mut Context) -> Result<()> {
        loop {
            if self.is_kind(TokenKind::Eof) {
                break Ok(());
            }

            context.register(self.parse_struct()?);
        }
    }

    fn parse_struct(&mut self) -> Result<StructNode> {
        self.consume_kind(TokenKind::Struct)?;

        let identifier = self.consume_kind(TokenKind::Identifier)?.value.to_owned();

        let mut extends = Vec::new();
        if self.is_kind(TokenKind::Colon) {
            self.consume()?;

            loop {
                extends.push(self.consume_kind(TokenKind::Identifier)?.value.to_owned());

                if self.is_kind(TokenKind::Plus) {
                    self.consume()?;
                } else {
                    break;
                }
            }
        }

        let mut fields = HashMap::new();

        self.consume_kind(TokenKind::CurlyOpen)?;
        loop {
            if !self.is_kind(TokenKind::Identifier) {
                break;
            }

            let field_name = self.consume_kind(TokenKind::Identifier)?.value.to_owned();

            self.consume_kind(TokenKind::Colon)?;

            fields.insert(field_name, self.parse_type()?);

            if self.is_kind(TokenKind::Comma) {
                self.consume()?;
            } else {
                break;
            }
        }
        self.consume_kind(TokenKind::CurlyClose)?;

        Ok(StructNode {
            identifier,
            extends,
            fields,
        })
    }

    fn parse_type(&mut self) -> Result<Box<dyn Node>> {
        let token = self.consume_kind(TokenKind::Identifier)?;

        let boxed: Box<dyn Node> = match token.value {
            "u8" => Box::new(PrimitiveNode::U8),
            "u16" => Box::new(PrimitiveNode::U16),
            "u32" => Box::new(PrimitiveNode::U32),
            "u64" => Box::new(PrimitiveNode::U64),
            "i8" => Box::new(PrimitiveNode::I8),
            "i16" => Box::new(PrimitiveNode::I16),
            "i32" => Box::new(PrimitiveNode::I32),
            "i64" => Box::new(PrimitiveNode::I64),
            "f32" => Box::new(PrimitiveNode::F32),
            "f64" => Box::new(PrimitiveNode::F64),
            "string" => Box::new(PrimitiveNode::String),
            "char" => Box::new(PrimitiveNode::Char),
            "_" => Box::new(PrimitiveNode::Unit),
            "Option" => Box::new({
                self.consume_kind(TokenKind::LessThan)?;
                let node = NativeNode::Option(self.parse_type()?);
                self.consume_kind(TokenKind::GreaterThan)?;
                node
            }),
            _ => Box::new(IdentifierNode {
                identifier: token.value.to_owned(),
            }),
        };

        Ok(boxed)
    }

    fn consume(&mut self) -> Result<Token<'s>> {
        let token = std::mem::replace(&mut self.current_token, self.lexer.next_token()?);
        println!("|> {:?}", token);
        Ok(token)
    }

    fn consume_kind(&mut self, kind: TokenKind) -> Result<Token<'s>> {
        if self.is_kind(kind) {
            Ok(self.consume()?)
        } else {
            Err(Error::Message(format!(
                "Expect {:?}, got {:?} at {}:{}",
                kind, self.current_token.kind, self.current_token.line, self.current_token.column
            )))
        }
    }

    fn is_kind(&self, kind: TokenKind) -> bool {
        self.current_token.kind == kind
    }
}
