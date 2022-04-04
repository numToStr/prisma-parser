use chumsky::{prelude::just, Parser};

use crate::{impl_parse, TokenType};

use super::terminal::{Id, Keyword, Name};

#[derive(Debug)]
pub struct Enum {
    pub this: Id,
    pub name: Name,
    pub variants: Variants,
}

impl_parse!(Enum, {
    just(TokenType::Enum)
        .map_with_span(|_, range| Id {
            value: Keyword::Enum,
            range,
        })
        .then(Name::parse())
        .then(Variants::parse())
        .map(|((this, name), variants)| Enum {
            this,
            name,
            variants,
        })
});

#[derive(Debug)]
pub struct Variants(Vec<Name>);

impl_parse!(Variants, {
    Name::parse()
        .repeated()
        .delimited_by(just(TokenType::OpenCurly), just(TokenType::CloseCurly))
        .map(Self)
});
