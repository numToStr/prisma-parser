use chumsky::{prelude::just, Parser};

use crate::{impl_parse, TokenType};

use super::{
    object::Fields,
    terminal::{Id, Keyword, Name},
};

#[derive(Debug)]
pub struct Generator {
    pub this: Id,
    pub name: Name,
    pub fields: Fields,
}

impl_parse!(Generator, {
    just(TokenType::Generator)
        .map_with_span(|_, s| Id {
            value: Keyword::Generator,
            range: s,
        })
        .then(Name::parse())
        .then(Fields::parse())
        .map(|((this, name), fields)| Self { this, name, fields })
});
