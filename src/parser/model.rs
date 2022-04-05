use chumsky::{
    prelude::{choice, just},
    Parser,
};

use crate::{impl_parse, Positioned, TokenType};

use super::{
    func::Func,
    terminal::{Keyword, Name, Scalar},
};

#[derive(Debug)]
pub struct Model {
    pub this: Positioned<Keyword>,
    pub name: Positioned<Name>,
    pub columns: Positioned<Columns>,
}

impl_parse!(Model, {
    just(TokenType::Model)
        .map_with_span(|_, range| Positioned::new(Keyword::Model, range))
        .then(Name::parse())
        .then(Columns::parse())
        .map_with_span(|((this, name), columns), range| {
            Positioned::new(
                Self {
                    this,
                    name,
                    columns,
                },
                range,
            )
        })
});

#[derive(Debug)]
pub struct Columns {
    pub value: Vec<Positioned<Column>>,
    pub attributes: Positioned<BlockAttrs>,
}

impl_parse!(Columns, {
    Column::parse()
        .repeated()
        .then(BlockAttrs::parse())
        .delimited_by(just(TokenType::OpenCurly), just(TokenType::CloseCurly))
        .map_with_span(|(value, attributes), range| {
            Positioned::new(Self { value, attributes }, range)
        })
});

#[derive(Debug)]
pub struct Column {
    pub name: Positioned<Name>,
    pub r#type: Positioned<ColumnType>,
    pub attributes: Positioned<Attributes>,
}

impl_parse!(Column, {
    Name::parse()
        .then(ColumnType::parse())
        .then(Attributes::parse())
        .map_with_span(|((name, t), attributes), range| {
            Positioned::new(
                Self {
                    name,
                    r#type: t,
                    attributes,
                },
                range,
            )
        })
});

#[derive(Debug)]
pub struct ColumnType {
    pub value: ScalarOrRef,
    pub modifier: Option<Positioned<Modifier>>,
}

impl_parse!(ColumnType, {
    ScalarOrRef::parse()
        .then(Modifier::parse())
        .map_with_span(|(value, modifier), range| Positioned::new(Self { value, modifier }, range))
});

#[derive(Debug)]
pub enum ScalarOrRef {
    Scalar(Positioned<Scalar>),
    Ref(Positioned<Name>),
}

impl_parse!(ScalarOrRef, Self, {
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
        .then(choice((
            Name::parse()
                .then(just(TokenType::Dot))
                .then(Property::parse())
                .map(|((name, _), property)| Self::Member { name, property }),
            Property::parse().map(Self::Simple),
        )))
        .map_with_span(|(_, node), range| Positioned::new(node, range))
});

#[derive(Debug)]
pub enum Property {
    Name(Positioned<Name>),
    Func(Positioned<Func>),
}

impl_parse!(Property, Self, {
    choice((Func::parse().map(Self::Func), Name::parse().map(Self::Name)))
});

#[derive(Debug)]
pub struct BlockAttrs(Vec<Positioned<Func>>);

impl_parse!(BlockAttrs, {
    just(TokenType::BlockAttr)
        .then(Func::parse())
        .map(|(_, func)| func)
        .repeated()
        .map_with_span(|value, range| Positioned::new(Self(value), range))
});
