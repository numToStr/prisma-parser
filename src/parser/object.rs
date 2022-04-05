use std::ops::Range;

use chumsky::{
    prelude::{choice, just},
    Parser,
};

use crate::{impl_parse, TokenType};

use super::{
    func::Func,
    terminal::{Name, Primary},
};

#[derive(Debug)]
pub struct Fields {
    pub value: Vec<Field>,
    pub range: Range<usize>,
}

impl_parse!(Fields, {
    Field::parse()
        .repeated()
        .delimited_by(just(TokenType::OpenCurly), just(TokenType::CloseCurly))
        .map_with_span(|value, range| Self { value, range })
});

#[derive(Debug)]
pub struct Field {
    pub key: Name,
    pub value: Value,
}

impl_parse!(Field, {
    Name::parse()
        .then_ignore(just(TokenType::Assign))
        .then(Value::parse())
        .map(|(key, value)| Self { key, value })
});

// FIXME: Maybe add function expression then remove Value
#[derive(Debug)]
pub enum Expr {
    Array(Array),
    Primary(Primary),
}

impl_parse!(Expr, {
    choice((
        Array::parse().map(Self::Array),
        Primary::parse().map(Self::Primary),
    ))
});

// TODO: interop with Expr
#[derive(Debug)]
pub enum Value {
    Expr(Expr),
    Func(Func),
}

impl_parse!(Value, {
    choice((Expr::parse().map(Self::Expr), Func::parse().map(Self::Func)))
});

#[derive(Debug)]
pub struct Array {
    pub items: Vec<ArrayItem>,
    pub range: Range<usize>,
}

impl_parse!(Array, {
    ArrayItem::parse()
        .separated_by(just(TokenType::Comma))
        .delimited_by(just(TokenType::OpenSquare), just(TokenType::CloseSquare))
        .map_with_span(|items, range| Self { items, range })
});

#[derive(Debug)]
pub enum ArrayItem {
    Primary(Primary),
    Ref(Name),
}

// NOTE: A neat thing about this function is that it won't allow mix datatypes
impl_parse!(ArrayItem, {
    choice((
        Name::parse().map(Self::Ref),
        Primary::parse().map(Self::Primary),
    ))
});
