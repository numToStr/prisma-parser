use std::ops::Range;

use chumsky::{
    prelude::{choice, filter_map, just, Simple},
    Error, Parser,
};

use crate::{impl_parse, TokenType};

#[derive(Debug)]
pub enum Keyword {
    DataSource,
    Generator,
    Model,
    Enum,
}

#[derive(Debug)]
pub struct Id {
    pub value: Keyword,
    pub range: Range<usize>,
}

#[derive(Debug)]
pub enum Primary {
    String { value: String, range: Range<usize> },
    Number { value: usize, range: Range<usize> },
    Bool { value: bool, range: Range<usize> },
}

impl_parse!(Primary, {
    filter_map(|s, t| match t {
        TokenType::Str(x) => Ok(Self::String { value: x, range: s }),
        TokenType::Num(x) => Ok(Self::Number { value: x, range: s }),
        TokenType::Bool(x) => Ok(Self::Bool { value: x, range: s }),
        _ => Err(Simple::expected_input_found(s, None, Some(t))),
    })
});

#[derive(Debug)]
pub struct Name {
    pub value: String,
    pub range: Range<usize>,
}

impl_parse!(Name, {
    filter_map(|s, t| match t {
        TokenType::Id(x) => Ok(Self { value: x, range: s }),
        _ => Err(Simple::expected_input_found(s, None, Some(t))),
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
});
