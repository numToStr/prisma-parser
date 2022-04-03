use crate::{Creator, Token, TokenType};

use super::terminal::Id;

// FIXME: missing open + close curly range
#[derive(Debug)]
pub struct Enum {
    pub this: Token,
    pub name: Id,
    pub variants: Variants,
}

impl Creator for Enum {
    fn create(tokens: &mut super::Tokens) -> super::PPResult<Self> {
        match tokens.next() {
            Some(x) => {
                let name = Id::create(tokens)?;
                let variants = Variants::create(tokens)?;

                Ok(Self {
                    this: x.clone(),
                    name,
                    variants,
                })
            }
            None => panic!("eof"),
        }
    }
}

#[derive(Debug)]
pub struct Variants(Vec<Id>);

impl Creator for Variants {
    fn create(tokens: &mut super::Tokens) -> super::PPResult<Self> {
        // open curly
        let _ = tokens.next().unwrap();

        let mut variants = vec![];

        loop {
            match tokens.peek() {
                Some(x) if x.ty == TokenType::CloseCurly => {
                    let _ = tokens.next().unwrap();

                    return Ok(Self(variants));
                }
                Some(_) => variants.push(Id::create(tokens)?),
                None => panic!("closing bracket missing"),
            };
        }
    }
}
