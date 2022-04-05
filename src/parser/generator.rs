use chumsky::{prelude::just, Parser};

use crate::{impl_parse, Positioned, TokenType};

use super::{
    object::Fields,
    terminal::{Keyword, Name},
};

#[derive(Debug)]
pub struct Generator {
    pub this: Positioned<Keyword>,
    pub name: Positioned<Name>,
    pub fields: Positioned<Fields>,
}

impl_parse!(Generator, {
    just(TokenType::Generator)
        .map_with_span(|_, range| Positioned::new(Keyword::Generator, range))
        .then(Name::parse())
        .then(Fields::parse())
        .map_with_span(|((this, name), fields), range| {
            Positioned::new(Self { this, name, fields }, range)
        })
});
