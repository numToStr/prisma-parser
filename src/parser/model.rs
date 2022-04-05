use std::ops::Range;

use chumsky::{
    prelude::{choice, just},
    Parser,
};

use crate::{impl_parse, TokenType};

use super::{
    func::Func,
    terminal::{Id, Keyword, Name, Scalar},
};

#[derive(Debug)]
pub struct Model {
    pub this: Id,
    pub name: Name,
    pub columns: Columns,
}

impl_parse!(Model, {
    just(TokenType::Model)
        .map_with_span(|_, range| Id {
            value: Keyword::Model,
            range,
        })
        .then(Name::parse())
        .then(Columns::parse())
        .map(|((this, name), columns)| Self {
            this,
            name,
            columns,
        })
});

#[derive(Debug)]
pub struct Columns {
    pub value: Vec<Column>,
    pub attribute: Option<BlockAttr>,
    pub range: Range<usize>,
}

impl_parse!(Columns, {
    Column::parse()
        .repeated()
        .then(BlockAttr::parse().or_not())
        .delimited_by(just(TokenType::OpenCurly), just(TokenType::CloseCurly))
        .map_with_span(|(value, attribute), range| Self {
            value,
            attribute,
            range,
        })
});

#[derive(Debug)]
pub struct Column {
    pub name: Name,
    pub r#type: ColumnType,
    pub attributes: Attributes,
    pub range: Range<usize>,
}

impl_parse!(Column, {
    Name::parse()
        .then(ColumnType::parse())
        .then(Attributes::parse())
        .map_with_span(|((name, t), attributes), range| Self {
            name,
            r#type: t,
            attributes,
            range,
        })
});

#[derive(Debug)]
pub struct ColumnType {
    pub value: ScalarOrRef,
    pub modifier: Option<Modifier>,
    pub range: Range<usize>,
}

impl_parse!(ColumnType, {
    ScalarOrRef::parse()
        .then(Modifier::parse())
        .map_with_span(|(value, modifier), range| Self {
            value,
            modifier,
            range,
        })
});

#[derive(Debug)]
pub enum ScalarOrRef {
    Scalar(Scalar),
    Ref(Name),
}

impl_parse!(ScalarOrRef, {
    Scalar::parse()
        .map(Self::Scalar)
        .or(Name::parse().map(Self::Ref))
});

#[derive(Debug, Clone)]
pub enum Modifier {
    Array,
    Optional,
}

impl_parse!(Modifier, Option<Self>, {
    choice((
        just(TokenType::Optional).to(Modifier::Optional),
        just(TokenType::OpenSquare)
            .then(just(TokenType::CloseSquare))
            .to(Modifier::Array),
    ))
    .or_not()
});

#[derive(Debug)]
pub struct Attributes {
    pub value: Vec<FieldAttr>,
    pub range: Range<usize>,
}

impl_parse!(Attributes, {
    FieldAttr::parse()
        .repeated()
        .map_with_span(|value, range| Self { value, range })
});

#[derive(Debug)]
pub struct FieldAttr {
    pub r#type: AttrType,
    pub range: Range<usize>,
}

impl_parse!(FieldAttr, {
    just(TokenType::FieldAttr)
        .then(AttrType::parse())
        .map_with_span(|(_, t), range| Self { r#type: t, range })
});

#[derive(Debug)]
pub enum AttrType {
    Simple(Property),
    Member { name: Name, property: Property },
}

impl_parse!(AttrType, {
    choice((
        Name::parse()
            .then(just(TokenType::Dot))
            .then(Property::parse())
            .map(|((name, _), property)| Self::Member { name, property }),
        Property::parse().map(Self::Simple),
    ))
});

#[derive(Debug)]
pub enum Property {
    Name(Name),
    Func(Func),
}

impl_parse!(Property, {
    choice((Func::parse().map(Self::Func), Name::parse().map(Self::Name)))
});

#[derive(Debug)]
pub struct BlockAttr {
    pub range: Range<usize>,
    pub value: Func,
}

impl_parse!(BlockAttr, {
    just(TokenType::BlockAttr)
        .then(Func::parse())
        .map_with_span(|(_, value), range| Self { value, range })
});
