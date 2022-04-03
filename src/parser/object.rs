use core::panic;

use crate::{Creator, Token, TokenType};

use super::{
    terminal::{Id, Primary},
    PPResult,
};

#[derive(Debug)]
pub struct Fields(Vec<Field>);

impl Creator for Fields {
    fn create(tokens: &mut super::Tokens) -> PPResult<Self> {
        // open curly
        let _ = tokens.next().unwrap();

        let mut fields = vec![];

        loop {
            match tokens.peek() {
                Some(Token {
                    ty: TokenType::CloseCurly,
                    ..
                }) => {
                    let _ = tokens.next().unwrap();

                    return Ok(Self(fields));
                }
                None => panic!("closing bracket missing"),
                Some(_) => fields.push(Field::create(tokens)?),
            };
        }

        // while let Some(
        //     x @ Token {
        //         ty: TokenType::RightCurly,
        //         ..
        //     },
        // ) = tokens.next()
        // {
        //     dbg!(x);
        // }
    }
}

#[derive(Debug)]
pub enum Value {
    Primary(Primary),
    Array(Array),
}

#[derive(Debug)]
pub struct Field {
    pub key: Id,
    pub value: Value,
}

impl Creator for Field {
    fn create(tokens: &mut super::Tokens) -> PPResult<Self> {
        let key = Id::create(tokens)?;

        // =
        tokens.next();

        let value = match tokens.peek() {
            Some(Token {
                ty: TokenType::Str(_) | TokenType::Num(_) | TokenType::Bool(_),
                ..
            }) => Value::Primary(Primary::create(tokens)?),
            Some(x) if x.ty == TokenType::OpenSquare => Value::Array(Array::create(tokens)?),
            _ => panic!("value not found"),
        };

        Ok(Field { key, value })
    }
}

#[derive(Debug)]
pub struct Array(Vec<ArrayItem>);

impl Creator for Array {
    fn create(tokens: &mut super::Tokens) -> PPResult<Self> {
        // open square
        let _ = tokens.next().unwrap();

        let mut items = vec![ArrayItem::create(tokens)?];

        loop {
            match tokens.next() {
                Some(x) if x.ty == TokenType::Comma => {
                    items.push(ArrayItem::create(tokens)?);
                }
                Some(x) if x.ty == TokenType::CloseSquare => return Ok(Self(items)),
                None => panic!("Closing square bracket not found"),
                x => panic!("Unexpected {:#?}", x),
            }
        }
    }
}

#[derive(Debug)]
pub enum ArrayItem {
    Ref(Id),
    Primary(Primary),
}

impl Creator for ArrayItem {
    fn create(tokens: &mut super::Tokens) -> PPResult<Self> {
        match tokens.peek() {
            Some(Token {
                ty: TokenType::Str(_) | TokenType::Num(_) | TokenType::Bool(_),
                ..
            }) => Ok(ArrayItem::Primary(Primary::create(tokens)?)),
            Some(Token {
                ty: TokenType::Id(_),
                ..
            }) => Ok(ArrayItem::Ref(Id::create(tokens)?)),
            _ => panic!("WTF"),
        }
    }
}
