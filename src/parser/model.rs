use chumsky::{
    prelude::{choice, just},
    Parser,
};

use crate::{impl_parse, Spanned, TokenType};

use super::{
    func::Call,
    terminal::{Keyword, Name, Scalar},
};

#[derive(Debug)]
pub struct Model {
    pub token: Spanned<Keyword>,
    pub name: Spanned<Name>,
    pub block: Spanned<Block>,
}

impl_parse!(Model, {
    just(TokenType::Model)
        .map_with_span(|_, range| Spanned::new(Keyword::Model, range))
        .then(Name::parse())
        .then(Block::parse())
        .map_with_span(|((token, name), block), range| {
            Spanned::new(Self { token, name, block }, range)
        })
});

#[derive(Debug)]
pub struct Block {
    pub columns: Spanned<Columns>,
    pub attributes: Spanned<BlockAttrs>,
}

impl_parse!(Block, {
    Columns::parse()
        .then(BlockAttrs::parse())
        .delimited_by(just(TokenType::OpenCurly), just(TokenType::CloseCurly))
        .map_with_span(|(columns, attributes), range| {
            Spanned::new(
                Self {
                    columns,
                    attributes,
                },
                range,
            )
        })
});

#[derive(Debug)]
pub struct Columns(Vec<Spanned<Column>>);

impl_parse!(Columns, {
    Column::parse()
        .repeated()
        .map_with_span(|node, range| Spanned::new(Self(node), range))
});

#[derive(Debug)]
pub struct Column {
    pub name: Spanned<Name>,
    pub scalar: Spanned<ColumnType>,
    pub attributes: Spanned<Attributes>,
}

impl_parse!(Column, {
    Name::parse()
        .then(ColumnType::parse())
        .then(Attributes::parse())
        .map_with_span(|((name, scalar), attributes), range| {
            Spanned::new(
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
    pub modifier: Option<Spanned<Modifier>>,
}

impl_parse!(ColumnType, {
    ScalarType::parse()
        .then(Modifier::parse())
        .map_with_span(|(value, modifier), range| Spanned::new(Self { value, modifier }, range))
});

#[derive(Debug)]
pub enum ScalarType {
    Scalar(Spanned<Scalar>),
    Ref(Spanned<Name>),
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

impl_parse!(Modifier, Option<Spanned<Self>>, {
    choice((
        just(TokenType::Optional).to(Modifier::Optional),
        just(TokenType::OpenSquare)
            .then(just(TokenType::CloseSquare))
            .to(Modifier::Array),
    ))
    .map_with_span(Spanned::new)
    .or_not()
});

#[derive(Debug)]
pub struct Attributes(Vec<Spanned<Attribute>>);

impl_parse!(Attributes, {
    Attribute::parse()
        .repeated()
        .map_with_span(|value, range| Spanned::new(Self(value), range))
});

#[derive(Debug)]
pub enum Attribute {
    Simple(Property),
    Member {
        name: Spanned<Name>,
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
        .map_with_span(Spanned::new)
});

#[derive(Debug)]
pub enum Property {
    Name(Spanned<Name>),
    Call(Spanned<Call>),
}

impl_parse!(Property, Self, {
    choice((Call::parse().map(Self::Call), Name::parse().map(Self::Name)))
});

#[derive(Debug)]
pub struct BlockAttrs(Vec<Spanned<BlockAttr>>);

impl_parse!(BlockAttrs, {
    BlockAttr::parse()
        .repeated()
        .map_with_span(|value, range| Spanned::new(Self(value), range))
});

#[derive(Debug)]
pub struct BlockAttr(Spanned<Call>);

impl_parse!(BlockAttr, {
    just(TokenType::BlockAttr)
        .ignore_then(Call::parse())
        .map_with_span(|value, range| Spanned::new(Self(value), range))
});
