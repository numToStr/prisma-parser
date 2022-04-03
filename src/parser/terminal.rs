use std::ops::Range;

use chumsky::{prelude::choice, select, Parser};

use crate::{impl_parse, TokenType};

#[derive(Debug)]
pub struct Token {
    pub ty: TokenType,
    pub range: Range<usize>,
}

#[derive(Debug)]
pub enum Primary {
    String(Token),
    Number(Token),
    Bool(Token),
}

// FIXME: somehow improve this
impl_parse!(Primary, {
    choice((
        select! { x @ TokenType::Str(_) => x }
            .map_with_span(|t, s| Self::String(Token { ty: t, range: s })),
        select! { x @ TokenType::Num(_) => x }
            .map_with_span(|t, s| Self::Number(Token { ty: t, range: s })),
        select! { x @ TokenType::Bool(_) => x }
            .map_with_span(|t, s| Self::Bool(Token { ty: t, range: s })),
    ))
});

// FIXME: maybe merge with `Token`
#[derive(Debug)]
pub struct Id(pub Token);

impl_parse!(Id, {
    select! {
        x @ TokenType::Id(_) => x
    }
    .map_with_span(|t, s| Id(Token { ty: t, range: s }))
});
