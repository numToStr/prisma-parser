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
        .map_with_span(|_, y| Id {
            value: Keyword::Model,
            range: y,
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
        .map_with_span(|v, r| Self { value: v, range: r })
});

#[derive(Debug)]
pub struct Column {
    pub key: Name,
    pub value: ColumnType,
    // pub directives
    pub range: Range<usize>,
}

impl_parse!(Column, {
    Name::parse()
        .then(ColumnType::parse())
        .map_with_span(|(key, value), r| Self {
            range: r,
            key,
            value,
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
        .map_with_span(|(v, m), r| Self {
            value: v,
            modifier: m,
            range: r,
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
