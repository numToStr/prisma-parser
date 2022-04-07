use chumsky::{
    prelude::{choice, just},
    Parser,
};

use crate::{impl_parse, Positioned, TokenType};

use super::{
    func::Call,
    terminal::{Keyword, Name, Scalar},
};

#[derive(Debug)]
pub struct Model {
    pub token: Positioned<Keyword>,
    pub name: Positioned<Name>,
    pub block: Positioned<Block>,
}

impl_parse!(Model, {
    just(TokenType::Model)
        .map_with_span(|_, range| Positioned::new(Keyword::Model, range))
        .then(Name::parse())
        .then(Block::parse())
        .map_with_span(|((token, name), block), range| {
            Positioned::new(Self { token, name, block }, range)
        })
});

#[derive(Debug)]
pub struct Block {
    pub columns: Positioned<Columns>,
    pub attributes: Positioned<BlockAttrs>,
}

impl_parse!(Block, {
    Columns::parse()
        .then(BlockAttrs::parse())
        .delimited_by(just(TokenType::OpenCurly), just(TokenType::CloseCurly))
        .map_with_span(|(columns, attributes), range| {
            Positioned::new(
                Self {
                    columns,
                    attributes,
                },
                range,
            )
        })
});

#[derive(Debug)]
pub struct Columns(Vec<Positioned<Column>>);

impl_parse!(Columns, {
    Column::parse()
        .repeated()
        .map_with_span(|node, range| Positioned::new(Self(node), range))
});

#[derive(Debug)]
pub struct Column {
    pub name: Positioned<Name>,
    pub scalar: Positioned<ColumnType>,
    pub attributes: Positioned<Attributes>,
}

impl_parse!(Column, {
    Name::parse()
        .then(ColumnType::parse())
        .then(Attributes::parse())
        .map_with_span(|((name, scalar), attributes), range| {
            Positioned::new(
                Self {
                    name,
                    scalar,
                    attributes,
                },
                range,
            )
        })
});

#[derive(Debug)]
pub struct ColumnType {
    pub value: ScalarType,
    pub modifier: Option<Positioned<Modifier>>,
}

impl_parse!(ColumnType, {
    ScalarType::parse()
        .then(Modifier::parse())
        .map_with_span(|(value, modifier), range| Positioned::new(Self { value, modifier }, range))
});

#[derive(Debug)]
pub enum ScalarType {
    Scalar(Positioned<Scalar>),
    Ref(Positioned<Name>),
}

impl_parse!(ScalarType, Self, {
    Scalar::parse()
        .map(Self::Scalar)
        .or(Name::parse().map(Self::Ref))
});

#[derive(Debug, Clone)]
pub enum Modifier {
    Array,
    Optional,
}

impl_parse!(Modifier, Option<Positioned<Self>>, {
    choice((
        just(TokenType::Optional).to(Modifier::Optional),
        just(TokenType::OpenSquare)
            .then(just(TokenType::CloseSquare))
            .to(Modifier::Array),
    ))
    .map_with_span(Positioned::new)
    .or_not()
});

#[derive(Debug)]
pub struct Attributes(Vec<Positioned<Attribute>>);

impl_parse!(Attributes, {
    Attribute::parse()
        .repeated()
        .map_with_span(|value, range| Positioned::new(Self(value), range))
});

#[derive(Debug)]
pub enum Attribute {
    Simple(Property),
    Member {
        name: Positioned<Name>,
        property: Property,
    },
}

impl_parse!(Attribute, {
    just(TokenType::FieldAttr)
        .ignore_then(choice((
            Name::parse()
                .then_ignore(just(TokenType::Dot))
                .then(Property::parse())
                .map(|(name, property)| Self::Member { name, property }),
            Property::parse().map(Self::Simple),
        )))
        .map_with_span(Positioned::new)
});

#[derive(Debug)]
pub enum Property {
    Name(Positioned<Name>),
    Call(Positioned<Call>),
}

impl_parse!(Property, Self, {
    choice((Call::parse().map(Self::Call), Name::parse().map(Self::Name)))
});

#[derive(Debug)]
pub struct BlockAttrs(Vec<Positioned<BlockAttr>>);

impl_parse!(BlockAttrs, {
    BlockAttr::parse()
        .repeated()
        .map_with_span(|value, range| Positioned::new(Self(value), range))
});

#[derive(Debug)]
pub struct BlockAttr(Positioned<Call>);

impl_parse!(BlockAttr, {
    just(TokenType::BlockAttr)
        .ignore_then(Call::parse())
        .map_with_span(|value, range| Positioned::new(Self(value), range))
});
