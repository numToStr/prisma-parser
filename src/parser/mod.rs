mod datasource;
mod r#enum;
mod func;
mod generator;
mod node;
mod object;
mod terminal;

use std::ops::Range;

use chumsky::{chain::Chain, prelude::Simple, Parser, Stream};

use crate::{parser::node::Node, Lexer, TokenType};

// Little helper macro for making parse function
#[macro_export]
macro_rules! impl_parse {
    ($id: ident, $ret: ty, $body: expr) => {
        impl $id {
            pub fn parse(
            ) -> impl chumsky::Parser<TokenType, $ret, Error = chumsky::prelude::Simple<TokenType>>
            {
                $body
            }
        }
    };
    ($id: ident, $body: expr) => {
        impl $id {
            pub fn parse(
            ) -> impl chumsky::Parser<TokenType, Self, Error = chumsky::prelude::Simple<TokenType>>
            {
                $body
            }
        }
    };
}

#[derive(Debug)]
pub struct Prisma {
    pub range: Range<usize>,
    pub nodes: Vec<Node>,
}

impl Prisma {
    pub fn parse(src: &str) -> Result<Self, Vec<Simple<TokenType>>> {
        let src_len = src.chars().len();
        let tokens = Lexer::parse(src).unwrap();
        let stream = Stream::from_iter(src_len..src_len + 1, tokens.into_iter());

        Node::parse()
            .repeated()
            // .recover_with(skip_then_retry_until([]))
            .map_with_span(|nodes, range| Self { nodes, range })
            .parse(stream)
    }
}
