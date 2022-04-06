use chumsky::{prelude::just, Parser};

use crate::{impl_parse, Positioned, TokenType};

use super::{
    object::Fields,
    terminal::{Keyword, Name},
};

#[derive(Debug)]
pub struct Datasource {
    pub token: Positioned<Keyword>,
    pub name: Positioned<Name>,
    pub fields: Positioned<Fields>,
}

impl_parse!(Datasource, {
    just(TokenType::DataSource)
        .map_with_span(|_, range| Positioned::new(Keyword::DataSource, range))
        .then(Name::parse())
        .then(Fields::parse())
        .map_with_span(|((token, name), fields), range| {
            Positioned::new(
                Self {
                    token,
                    name,
                    fields,
                },
                range,
            )
        })
});
