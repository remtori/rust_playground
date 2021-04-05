use std::collections::HashMap;

use crate::{context::{Context, FieldDeclaration, Message, TypeDeclaration}, error::{Error, ParseError, Result}, lexer::Lexer, token::{Token, TokenKind}};

pub struct Parser<'s> {
    lexer: Lexer<'s>,
    current_token: Token<'s>,
    pub(crate) context: Context<'s>,
}

impl<'s> Parser<'s> {
    pub fn new(source: &'s str) -> Parser<'s> {
        let mut lexer = Lexer::new(source);
        let current_token = lexer.next_token().unwrap();

        Parser {
            lexer,
            context: Default::default(),
            current_token,
        }
    }

    pub fn parse_schema(&mut self) -> Result<()> {
        loop {
            if self.is_kind(TokenKind::Eof) {
                break Ok(());
            }

            let msg = self.parse_message()?;
            self.context.register(msg);
        }
    }

    fn parse_message(&mut self) -> Result<Message<'s>> {
        self.consume_kind(TokenKind::Message)?;

        let identifier = self.parse_type()?;

        let mut extends = Vec::new();
        if self.is_kind(TokenKind::Colon) {
            self.consume()?;

            loop {
                extends.push(self.consume_kind(TokenKind::Identifier)?.value);

                if self.is_kind(TokenKind::Plus) {
                    self.consume()?;
                } else {
                    break;
                }
            }
        }

        let mut fields = Vec::new();

        self.consume_kind(TokenKind::CurlyOpen)?;
        loop {
            if !self.is_kind(TokenKind::Identifier) {
                break;
            }

            let field_name = self.consume_kind(TokenKind::Identifier)?.value;

            self.consume_kind(TokenKind::Colon)?;

            let ty = self.parse_type()?;

            self.consume_kind(TokenKind::Equals)?;
            
            let id = self.consume_kind(TokenKind::IntegerLiteral)?;

            fields.push(FieldDeclaration { 
                id: id.parse::<u32>()?,
                ident: field_name, 
                ty,
            });

            if self.is_kind(TokenKind::Semicolon) {
                self.consume()?;
            }
            
            if self.is_kind(TokenKind::CurlyClose) {
                break;
            }
        }
        self.consume_kind(TokenKind::CurlyClose)?;

        Ok(Message {
            identifier,
            extends,
            fields,
        })
    }

    fn parse_type(&mut self) -> Result<TypeDeclaration<'s>> {
        let ident = self.consume_kind(TokenKind::Identifier)?;
        let mut generics = Vec::new();

        if self.is_kind(TokenKind::LessThan) {
            self.consume()?;

            loop {
                let ty = self.consume_kind(TokenKind::Identifier)?;
                generics.push(ty.value);

                if self.is_kind(TokenKind::Comma) {
                    self.consume()?;
                } else {
                    break;
                }
            }

            self.consume_kind(TokenKind::GreaterThan)?;
        }
        

        Ok(TypeDeclaration {
            ident: ident.value,
            generics
        })
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
