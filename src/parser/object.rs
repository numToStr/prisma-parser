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
pub struct Fields(Vec<Field>);

impl_parse!(Fields, {
    Field::parse()
        .repeated()
        .delimited_by(just(TokenType::OpenCurly), just(TokenType::CloseCurly))
        .map(Self)
});

#[derive(Debug)]
pub enum Value {
    Primary(Primary),
    Array(Array),
    Func(Func),
}

impl_parse!(Value, {
    choice((
        Primary::parse().map(Self::Primary),
        Array::parse().map(Self::Array),
        Func::parse().map(Self::Func),
    ))
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

#[derive(Debug)]
pub struct Array(Vec<ArrayItem>);

impl_parse!(Array, {
    ArrayItem::parse()
        // .separated_by(just(TokenType::Comma))
        .delimited_by(just(TokenType::OpenSquare), just(TokenType::CloseSquare))
        .map(Self)
});

#[derive(Debug)]
pub enum ArrayItem {
    Ref(Name),
    Primary(Primary),
}

// NOTE: A neat thing about this function is that it won't allow mix datatypes
impl_parse!(ArrayItem, Vec<ArrayItem>, {
    choice((
        Primary::parse()
            .map(Self::Primary)
            .separated_by(just(TokenType::Comma)),
        Name::parse()
            .map(Self::Ref)
            .separated_by(just(TokenType::Comma)),
    ))
});
