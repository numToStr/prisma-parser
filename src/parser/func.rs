use std::ops::Range;

use chumsky::{
    prelude::{choice, just},
    Parser,
};

use crate::{impl_parse, TokenType};

use super::{
    object::Array,
    terminal::{Name, Primary},
};

#[derive(Debug)]
pub struct Func {
    pub name: Name,
    pub args: Args,
    pub range: Range<usize>,
}

impl_parse!(Func, {
    Name::parse()
        .then(Args::parse())
        .map_with_span(|(x, y), r| Self {
            name: x,
            args: y,
            range: r,
        })
});

#[derive(Debug)]
pub struct Args {
    pub value: Vec<Arg>,
    pub range: Range<usize>,
}

impl_parse!(Args, {
    Arg::parse()
        .separated_by(just(TokenType::Comma))
        .delimited_by(just(TokenType::OpenParen), just(TokenType::CloseParen))
        .map_with_span(|v, r| Self { value: v, range: r })
});

#[derive(Debug)]
pub enum Arg {
    Primary(Primary),
    Array(Array),
    // Func(Func),
    // Named(Named)
}

impl_parse!(Arg, {
    choice((
        Primary::parse().map(Self::Primary),
        Array::parse().map(Self::Array),
        // Func::parse().map(Self::Func),
    ))
});
