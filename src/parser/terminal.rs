use chumsky::{
    prelude::{choice, filter_map, just, Simple},
    select, Error, Parser,
};

use crate::{impl_parse, Spanned, TokenType};

#[derive(Debug)]
pub enum Keyword {
    DataSource,
    Generator,
    Model,
    Enum,
}

#[derive(Debug)]
pub enum Literal {
    String(String),
    Number(usize),
    Bool(bool),
}

impl_parse!(Literal, {
    filter_map(|range, token| match token {
        TokenType::Str(value) => Ok(Self::String(value)),
        TokenType::Num(value) => Ok(Self::Number(value)),
        TokenType::Bool(value) => Ok(Self::Bool(value)),
        _ => Err(Simple::expected_input_found(range, None, Some(token))),
    })
    .map_with_span(Spanned::new)
});

#[derive(Debug)]
pub struct Name(String);

impl_parse!(Name, {
    filter_map(|range, token| match token {
        TokenType::Id(value) => Ok(Spanned::new(Self(value), range)),
        _ => Err(Simple::expected_input_found(range, None, Some(token))),
    })
});

#[derive(Debug, Clone)]
pub enum Scalar {
    String,
    Boolean,
    Int,
    BigInt,
    Float,
    Decimal,
    DateTime,
    Json,
    Bytes,
    Unsupported(String),
}

impl_parse!(Scalar, {
    choice((
        just(TokenType::String).to(Self::String),
        just(TokenType::Boolean).to(Self::Boolean),
        just(TokenType::Int).to(Self::Int),
        just(TokenType::BigInt).to(Self::BigInt),
        just(TokenType::Float).to(Self::Float),
        just(TokenType::Decimal).to(Self::Decimal),
        just(TokenType::DateTime).to(Self::DateTime),
        just(TokenType::Json).to(Self::Json),
        just(TokenType::Bytes).to(Self::Bytes),
        just(TokenType::Unsupported).ignore_then(
            select! {
                TokenType::Str(x) => Self::Unsupported(x)
            }
            .delimited_by(just(TokenType::OpenParen), just(TokenType::CloseParen)),
        ),
    ))
    .map_with_span(Spanned::new)
});
