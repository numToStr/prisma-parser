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
    filter_map(|range, token| match token {
        TokenType::Str(value) => Ok(Self::String { value, range }),
        TokenType::Num(value) => Ok(Self::Number { value, range }),
        TokenType::Bool(value) => Ok(Self::Bool { value, range }),
        _ => Err(Simple::expected_input_found(range, None, Some(token))),
    })
});

#[derive(Debug)]
pub struct Name {
    pub value: String,
    pub range: Range<usize>,
}

impl_parse!(Name, {
    filter_map(|range, token| match token {
        TokenType::Id(value) => Ok(Self { value, range }),
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
});
