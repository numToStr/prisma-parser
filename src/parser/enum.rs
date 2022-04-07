use chumsky::{prelude::just, Parser};

use crate::{impl_parse, Spanned, TokenType};

use super::terminal::{Keyword, Name};

#[derive(Debug)]
pub struct Enum {
    pub token: Spanned<Keyword>,
    pub name: Spanned<Name>,
    pub variants: Spanned<Variants>,
}

impl_parse!(Enum, {
    just(TokenType::Enum)
        .map_with_span(|_, range| Spanned::new(Keyword::Enum, range))
        .then(Name::parse())
        .then(Variants::parse())
        .map(|((token, name), variants)| Enum {
            token,
            name,
            variants,
        })
        .map_with_span(Spanned::new)
});

#[derive(Debug)]
pub struct Variants(Vec<Spanned<Name>>);

impl_parse!(Variants, {
    Name::parse()
        .repeated()
        .delimited_by(just(TokenType::OpenCurly), just(TokenType::CloseCurly))
        .map_with_span(|node, range| Spanned::new(Self(node), range))
});
