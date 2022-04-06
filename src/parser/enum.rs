use chumsky::{prelude::just, Parser};

use crate::{impl_parse, Positioned, TokenType};

use super::terminal::{Keyword, Name};

#[derive(Debug)]
pub struct Enum {
    pub token: Positioned<Keyword>,
    pub name: Positioned<Name>,
    pub variants: Positioned<Variants>,
}

impl_parse!(Enum, {
    just(TokenType::Enum)
        .map_with_span(|_, range| Positioned::new(Keyword::Enum, range))
        .then(Name::parse())
        .then(Variants::parse())
        .map(|((token, name), variants)| Enum {
            token,
            name,
            variants,
        })
        .map_with_span(Positioned::new)
});

#[derive(Debug)]
pub struct Variants(Vec<Positioned<Name>>);

impl_parse!(Variants, {
    Name::parse()
        .repeated()
        .delimited_by(just(TokenType::OpenCurly), just(TokenType::CloseCurly))
        .map_with_span(|node, range| Positioned::new(Self(node), range))
});
