use chumsky::{
    prelude::{choice, just},
    Parser,
};

use crate::{impl_parse, TokenType};

use super::terminal::{Id, Primary};

#[derive(Debug)]
pub struct Func {
    pub name: Id,
    pub args: Args,
}

impl_parse!(Func, {
    Id::parse()
        .then(Args::parse())
        .map(|(x, y)| Self { name: x, args: y })
});

#[derive(Debug)]
pub struct Args(Vec<Arg>);

impl_parse!(Args, {
    Arg::parse()
        .delimited_by(just(TokenType::OpenParen), just(TokenType::CloseParen))
        .map(Self)
});

#[derive(Debug)]
pub enum Arg {
    Primary(Primary),
    // Named{name: Id, value: Value}
}

impl_parse!(Arg, Vec<Arg>, {
    choice((
        Primary::parse()
            .map(Self::Primary)
            .separated_by(just(TokenType::Comma)),
        // Value::parse()
        //     .map(Self::Ref)
        //     .separated_by(just(TokenType::Comma)),
    ))
});
