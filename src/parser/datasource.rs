use chumsky::{prelude::just, Parser};

use crate::{impl_parse, TokenType};

use super::{
    object::Fields,
    terminal::{Id, Token},
};

#[derive(Debug)]
pub struct Datasource {
    pub this: Token,
    pub name: Id,
    pub fields: Fields,
}

impl_parse!(Datasource, {
    just(TokenType::DataSource)
        .map_with_span(|t, s| Token { ty: t, range: s })
        .then(Id::parse())
        .then(Fields::parse())
        .map(|((this, name), fields)| Self { this, name, fields })
});
