use std::ops::Range;

use chumsky::{
    prelude::{choice, Simple},
    Parser,
};

use crate::{impl_parse, TokenType};

use super::{datasource::Datasource, generator::Generator};

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
}

impl_parse!(Node, {
    choice::<_, Simple<TokenType>>((
        Datasource::parse().map_with_span(|n, r| Node::Datasource { node: n, range: r }),
        Generator::parse().map_with_span(|n, r| Node::Generator { node: n, range: r }),
    ))
});
