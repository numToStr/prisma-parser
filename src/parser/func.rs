use crate::Token;

use super::{
    object::Value,
    terminal::{Id, Primary},
};

#[derive(Debug)]
pub struct Func {
    pub this: Token,
    pub name: Id,
    pub args: Args,
}

#[derive(Debug)]
pub struct Args(Vec<Arg>);

#[derive(Debug)]
pub enum Arg {
    Literal(Primary),
    Named(Named),
}

#[derive(Debug)]
pub struct Named {
    pub name: Id,
    pub value: Value,
}
