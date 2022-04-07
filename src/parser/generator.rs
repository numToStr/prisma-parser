use chumsky::{prelude::just, Parser};

use crate::{impl_parse, Spanned, TokenType};

use super::{
    object::Fields,
    terminal::{Keyword, Name},
};

#[derive(Debug)]
pub struct Generator {
    pub token: Spanned<Keyword>,
    pub name: Spanned<Name>,
    pub fields: Spanned<Fields>,
}

impl_parse!(Generator, {
    just(TokenType::Generator)
        .map_with_span(|_, range| Spanned::new(Keyword::Generator, range))
        .then(Name::parse())
        .then(Fields::parse())
        .map_with_span(|((token, name), fields), range| {
            Spanned::new(
                Self {
                    token,
                    name,
                    fields,
                },
                range,
            )
        })
});
