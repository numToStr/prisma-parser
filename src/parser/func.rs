use chumsky::{
    prelude::{choice, just, recursive},
    Parser,
};

use crate::{impl_parse, object::Array, terminal::Literal, Spanned, TokenType};

use super::terminal::Name;

#[derive(Debug)]
pub struct Call {
    pub name: Spanned<Name>,
    pub args: Spanned<Args>,
}

#[derive(Debug)]
pub struct Args(Vec<Arg>);

#[derive(Debug)]
pub enum Arg {
    Array(Spanned<Array>),
    Literal(Spanned<Literal>),
    Named(Spanned<Named>),
    Ref(Spanned<Name>),
    Call(Spanned<Call>),
}

impl_parse!(Call, {
    recursive(|x| {
        Name::parse()
            .then(
                choice((
                    x.map(Arg::Call),
                    Literal::parse().map(Arg::Literal),
                    Array::parse().map(Arg::Array),
                    Named::parse().map(Arg::Named),
                    Name::parse().map(Arg::Ref),
                ))
                .separated_by(just(TokenType::Comma))
                .map_with_span(|node, range| Spanned::new(Args(node), range))
                .delimited_by(just(TokenType::OpenParen), just(TokenType::CloseParen)),
            )
            .map_with_span(|(name, args), range| Spanned::new(Self { name, args }, range))
    })
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
