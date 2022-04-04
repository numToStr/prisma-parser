use std::ops::Range;

use chumsky::{
    prelude::{choice, Simple},
    Parser,
};

use crate::{impl_parse, TokenType};

use super::{datasource::Datasource, generator::Generator, model::Model, r#enum::Enum};

#[derive(Debug)]
pub enum Node {
    Datasource {
        node: Datasource,
        range: Range<usize>,
    },
    Generator {
        node: Generator,
        range: Range<usize>,
    },
    Enum {
        node: Enum,
        range: Range<usize>,
    },
    Model {
        node: Model,
        range: Range<usize>,
    },
}

impl_parse!(Node, {
    choice::<_, Simple<TokenType>>((
        Datasource::parse().map_with_span(|node, range| Node::Datasource { node, range }),
        Generator::parse().map_with_span(|node, range| Node::Generator { node, range }),
        Enum::parse().map_with_span(|node, range| Node::Enum { node, range }),
        Model::parse().map_with_span(|node, range| Node::Model { node, range }),
    ))
});
