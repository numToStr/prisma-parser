use crate::{Creator, Token, TokenType};

use super::PPResult;

#[derive(Debug)]
pub struct Primary(pub Token);

impl Creator for Primary {
    fn create(tokens: &mut super::Tokens) -> PPResult<Self> {
        match tokens.next() {
            Some(
                x @ Token {
                    ty: TokenType::Str(_) | TokenType::Num(_) | TokenType::Bool(_),
                    ..
                },
            ) => Ok(Self(x.clone())),
            None => panic!("EOF"),
            x => panic!("Unexpected {:#?}", x),
        }
    }
}

#[derive(Debug)]
pub struct Id(pub Token);

impl Creator for Id {
    fn create(tokens: &mut super::Tokens) -> PPResult<Self> {
        match tokens.next() {
            Some(
                x @ Token {
                    ty: TokenType::Id(_),
                    ..
                },
            ) => Ok(Self(x.clone())),
            None => panic!("EOF"),
            x => panic!("Unexpected {:#?}", x),
        }
    }
}
