use std::ops::Range;

use chumsky::{
    prelude::{choice, just},
    Parser,
};

use crate::{impl_parse, TokenType};

use super::terminal::{Id, Keyword, Name, Scalar};

#[derive(Debug)]
pub struct Model {
    pub this: Id,
    pub name: Name,
    pub columns: Columns,
}

impl_parse!(Model, {
    just(TokenType::Model)
        .map_with_span(|_, range| Id {
            value: Keyword::Model,
            range,
        })
        .then(Name::parse())
        .then(Columns::parse())
        .map(|((this, name), columns)| Self {
            this,
            name,
            columns,
        })
});

#[derive(Debug)]
pub struct Columns {
    pub value: Vec<Column>,
    pub range: Range<usize>,
}

impl_parse!(Columns, {
    Column::parse()
        .repeated()
        .delimited_by(just(TokenType::OpenCurly), just(TokenType::CloseCurly))
        .map_with_span(|value, range| Self { value, range })
});

#[derive(Debug)]
pub struct Column {
    pub name: Name,
    pub r#type: ColumnType,
    // pub directives
    pub range: Range<usize>,
}

impl_parse!(Column, {
    Name::parse()
        .then(ColumnType::parse())
        .map_with_span(|(name, t), range| Self {
            name,
            r#type: t,
            range,
        })
});

#[derive(Debug)]
pub struct ColumnType {
    pub value: Type,
    pub modifier: Option<Modifier>,
    pub range: Range<usize>,
}

impl_parse!(ColumnType, {
    Type::parse()
        .then(Modifier::parse())
        .map_with_span(|(value, modifier), range| Self {
            value,
            modifier,
            range,
        })
});

#[derive(Debug)]
pub enum Type {
    Scalar(Scalar),
    Ref(Name),
}

impl_parse!(Type, {
    Scalar::parse()
        .map(Self::Scalar)
        .or(Name::parse().map(Self::Ref))
});

#[derive(Debug, Clone)]
pub enum Modifier {
    Array,
    Optional,
}

impl_parse!(Modifier, Option<Self>, {
    choice((
        just(TokenType::Optional).to(Modifier::Optional),
        just(TokenType::OpenSquare)
            .then(just(TokenType::CloseSquare))
            .to(Modifier::Array),
    ))
    .or_not()
});
