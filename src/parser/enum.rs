use chumsky::{prelude::just, Parser};

use crate::{impl_parse, TokenType};

use super::terminal::{Id, Token};

#[derive(Debug)]
pub struct Enum {
    pub this: Token,
    pub name: Id,
    pub variants: Variants,
}

impl_parse!(Enum, {
    just(TokenType::Enum)
        .map_with_span(|x, y| Token { ty: x, range: y })
        .then(Id::parse())
        .then(Variants::parse())
        .map(|((this, name), variants)| Enum {
            this,
            name,
            variants,
        })
});

#[derive(Debug)]
pub struct Variants(Vec<Id>);

impl_parse!(Variants, {
    Id::parse()
        .repeated()
        .delimited_by(just(TokenType::OpenCurly), just(TokenType::CloseCurly))
        .map(Self)
});
