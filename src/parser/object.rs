use chumsky::{
    prelude::{choice, just},
    Parser,
};

use crate::{impl_parse, Positioned, TokenType};

use super::{
    func::Func,
    terminal::{Name, Primary},
};

#[derive(Debug)]
pub struct Fields(Vec<Positioned<Field>>);

impl_parse!(Fields, {
    Field::parse()
        .repeated()
        .delimited_by(just(TokenType::OpenCurly), just(TokenType::CloseCurly))
        .map_with_span(|value, range| Positioned::new(Self(value), range))
});

#[derive(Debug)]
pub struct Field {
    pub key: Positioned<Name>,
    pub value: Value,
}

impl_parse!(Field, {
    Name::parse()
        .then_ignore(just(TokenType::Assign))
        .then(Value::parse())
        .map_with_span(|(key, value), range| Positioned::new(Self { key, value }, range))
});

// FIXME: Maybe add function expression then remove Value
#[derive(Debug)]
pub enum Expr {
    Array(Positioned<Array>),
    Primary(Positioned<Primary>),
}

impl_parse!(Expr, Self, {
    choice((
        Array::parse().map(Self::Array),
        Primary::parse().map(Self::Primary),
    ))
});

// TODO: interop with Expr
#[derive(Debug)]
pub enum Value {
    Expr(Expr),
    Func(Positioned<Func>),
}

impl_parse!(Value, Self, {
    choice((Expr::parse().map(Self::Expr), Func::parse().map(Self::Func)))
});

#[derive(Debug)]
pub struct Array(Vec<ArrayItem>);

impl_parse!(Array, {
    ArrayItem::parse()
        .separated_by(just(TokenType::Comma))
        .delimited_by(just(TokenType::OpenSquare), just(TokenType::CloseSquare))
        .map_with_span(|items, range| Positioned::new(Self(items), range))
});

#[derive(Debug)]
pub enum ArrayItem {
    Primary(Positioned<Primary>),
    Ref(Positioned<Name>),
}

// NOTE: A neat thing about this function is that it won't allow mix datatypes
impl_parse!(ArrayItem, Self, {
    choice((
        Name::parse().map(Self::Ref),
        Primary::parse().map(Self::Primary),
    ))
});
