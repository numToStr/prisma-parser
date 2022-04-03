use crate::{Creator, Token};

use super::{object::Fields, terminal::Id, PPResult};

#[derive(Debug)]
pub struct Generator {
    pub this: Token,
    pub name: Id,
    pub fields: Fields,
}

impl Creator for Generator {
    fn create(tokens: &mut super::Tokens) -> PPResult<Self> {
        match tokens.next() {
            Some(x) => {
                let name = Id::create(tokens)?;
                let fields = Fields::create(tokens)?;

                Ok(Self {
                    this: x.clone(),
                    name,
                    fields,
                })
            }
            None => panic!("EOF"),
        }
    }
}