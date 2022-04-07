use chumsky::{
    prelude::{choice, just},
    Parser,
};

use crate::{impl_parse, Spanned, TokenType};

use super::{object::Expr, terminal::Name};

#[derive(Debug)]
pub struct Call {
    pub name: Spanned<Name>,
    pub args: Spanned<Args>,
}

impl_parse!(Call, {
    Name::parse()
        .then(Args::parse())
        .map_with_span(|(name, args), range| Spanned::new(Self { name, args }, range))
});

#[derive(Debug)]
pub struct Args(Vec<Arg>);

impl_parse!(Args, {
    Arg::parse()
        .separated_by(just(TokenType::Comma))
        .delimited_by(just(TokenType::OpenParen), just(TokenType::CloseParen))
        .map_with_span(|value, range| Spanned::new(Self(value), range))
});

// FIXME: handle function
#[derive(Debug)]
pub enum Arg {
    Expr(Expr),
    Named(Spanned<Named>),
    Ref(Spanned<Name>),
    // Call(Call),
}

impl_parse!(Arg, Self, {
    choice((
        Expr::parse().map(Self::Expr),
        Named::parse().map(Self::Named),
        Name::parse().map(Self::Ref),
        // Call::parse().map(Self::Call),
    ))
});

#[derive(Debug)]
pub struct Named {
    pub key: Spanned<Name>,
    pub value: Expr,
}

impl_parse!(Named, {
    Name::parse()
        .then_ignore(just(TokenType::Colon))
        .then(Expr::parse())
        .map_with_span(|(key, value), range| Spanned::new(Self { key, value }, range))
});
