use chumsky::{
    prelude::{choice, just},
    Parser,
};

use crate::{impl_parse, Spanned, TokenType};

use super::{
    func::Call,
    terminal::{Literal, Name},
};

#[derive(Debug)]
pub struct Fields(Vec<Spanned<Field>>);

impl_parse!(Fields, {
    Field::parse()
        .repeated()
        .delimited_by(just(TokenType::OpenCurly), just(TokenType::CloseCurly))
        .map_with_span(|value, range| Spanned::new(Self(value), range))
});

#[derive(Debug)]
pub struct Field {
    pub key: Spanned<Name>,
    pub value: Value,
}

impl_parse!(Field, {
    Name::parse()
        .then_ignore(just(TokenType::Assign))
        .then(Value::parse())
        .map_with_span(|(key, value), range| Spanned::new(Self { key, value }, range))
});

// FIXME: Maybe add function expression then remove Value
#[derive(Debug)]
pub enum Expr {
    Array(Spanned<Array>),
    Literal(Spanned<Literal>),
}

impl_parse!(Expr, Self, {
    choice((
        Array::parse().map(Self::Array),
        Literal::parse().map(Self::Literal),
    ))
});

// TODO: interop with Expr
#[derive(Debug)]
pub enum Value {
    Expr(Expr),
    Call(Spanned<Call>),
}

impl_parse!(Value, Self, {
    choice((Expr::parse().map(Self::Expr), Call::parse().map(Self::Call)))
});

#[derive(Debug)]
pub struct Array(Vec<ArrayItem>);

impl_parse!(Array, {
    ArrayItem::parse()
        .separated_by(just(TokenType::Comma))
        .delimited_by(just(TokenType::OpenSquare), just(TokenType::CloseSquare))
        .map_with_span(|items, range| Spanned::new(Self(items), range))
});

#[derive(Debug)]
pub enum ArrayItem {
    Literal(Spanned<Literal>),
    Ref(Spanned<Name>),
}

// NOTE: A neat thing about this function is that it won't allow mix datatypes
impl_parse!(ArrayItem, Self, {
    choice((
        Name::parse().map(Self::Ref),
        Literal::parse().map(Self::Literal),
    ))
});
