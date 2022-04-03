mod datasource;
mod r#enum;
mod func;
mod generator;
mod node;
mod object;
mod terminal;

use std::{error::Error, iter::Peekable, slice::Iter};

use crate::{Token, TokenType};

use self::{
    datasource::Datasource,
    generator::Generator,
    node::{Document, Node},
    r#enum::Enum,
};

type Tokens<'p> = Peekable<Iter<'p, Token>>;

type PPResult<T> = Result<T, Box<dyn Error>>;

pub(super) trait Creator {
    fn create(tokens: &mut Tokens) -> PPResult<Self>
    where
        Self: std::marker::Sized;
}

#[derive(Debug)]
pub struct Parser;

impl Parser {
    pub fn parse(spans: &[Token]) -> PPResult<Document> {
        let mut document = Document::default();
        let mut tokens = spans.iter().peekable();

        loop {
            match tokens.peek() {
                Some(x) if x.ty == TokenType::DataSource => {
                    document.add_node(Node::Datasource(Datasource::create(&mut tokens)?))
                }
                Some(x) if x.ty == TokenType::Generator => {
                    document.add_node(Node::Generator(Generator::create(&mut tokens)?))
                }
                Some(x) if x.ty == TokenType::Enum => {
                    document.add_node(Node::Enum(Enum::create(&mut tokens)?))
                }
                Some(Token {
                    ty: TokenType::Model,
                    ..
                }) => {
                    tokens.next();
                }
                None => break,
                _ => {
                    panic!("Unexpected token: {:#?}", tokens.next())
                }
            }
        }

        Ok(document)
    }
}
