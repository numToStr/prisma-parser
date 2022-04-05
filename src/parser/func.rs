use std::ops::Range;

use chumsky::{
    prelude::{choice, just},
    Parser,
};

use crate::{impl_parse, TokenType};

use super::{object::Expr, terminal::Name};

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

// FIXME: handle function
#[derive(Debug)]
pub enum Arg {
    Expr(Expr),
    Named(Named),
    Ref(Name),
    // Func(Func),
}

impl_parse!(Arg, {
    choice((
        Expr::parse().map(Self::Expr),
        Named::parse().map(Self::Named),
        Name::parse().map(Self::Ref),
        // Func::parse().map(Self::Func),
    ))
});

#[derive(Debug)]
pub struct Named {
    pub key: Name,
    pub value: Expr,
    pub range: Range<usize>,
}

impl_parse!(Named, {
    Name::parse()
        .then(just(TokenType::Colon))
        .then(Expr::parse())
        .map_with_span(|((key, _), value), range| Self { key, value, range })
});
