use chumsky::{prelude::choice, Parser};

use crate::{impl_parse, Positioned, TokenType};

use super::{datasource::Datasource, generator::Generator, model::Model, r#enum::Enum};

#[derive(Debug)]
pub enum Node {
    Datasource(Positioned<Datasource>),
    Generator(Positioned<Generator>),
    Model(Positioned<Model>),
    Enum(Positioned<Enum>),
}

impl_parse!(Node, Self, {
    choice((
        Datasource::parse().map(Node::Datasource),
        Generator::parse().map(Node::Generator),
        Model::parse().map(Node::Model),
        Enum::parse().map(Node::Enum),
    ))
});
