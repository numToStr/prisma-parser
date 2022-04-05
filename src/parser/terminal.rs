use chumsky::{
    prelude::{choice, filter_map, just, Simple},
    Error, Parser,
};

use crate::{impl_parse, Positioned, TokenType};

#[derive(Debug)]
pub enum Keyword {
    DataSource,
    Generator,
    Model,
    Enum,
}

#[derive(Debug)]
pub enum Primary {
    String(String),
    Number(usize),
    Bool(bool),
}

impl_parse!(Primary, {
    filter_map(|range, token| match token {
        TokenType::Str(value) => Ok(Self::String(value)),
        TokenType::Num(value) => Ok(Self::Number(value)),
        TokenType::Bool(value) => Ok(Self::Bool(value)),
        _ => Err(Simple::expected_input_found(range, None, Some(token))),
    })
    .map_with_span(Positioned::new)
});

#[derive(Debug)]
pub struct Name(String);

impl_parse!(Name, {
    filter_map(|range, token| match token {
        TokenType::Id(value) => Ok(Positioned::new(Self(value), range)),
        _ => Err(Simple::expected_input_found(range, None, Some(token))),
    })
});

// FIXME: handle `Unsupported`
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
    // Unsupported
}

impl_parse!(Scalar, {
    choice((
        just(TokenType::String).to(Scalar::String),
        just(TokenType::Boolean).to(Scalar::Boolean),
        just(TokenType::Int).to(Scalar::Int),
        just(TokenType::BigInt).to(Scalar::BigInt),
        just(TokenType::Float).to(Scalar::Float),
        just(TokenType::Decimal).to(Scalar::Decimal),
        just(TokenType::DateTime).to(Scalar::DateTime),
        just(TokenType::Json).to(Scalar::Json),
        just(TokenType::Bytes).to(Scalar::Bytes),
    ))
    .map_with_span(Positioned::new)
});
