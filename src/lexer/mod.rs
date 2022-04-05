mod token_type;
pub use token_type::*;

use std::ops::Range;

use chumsky::{
    prelude::{choice, filter, just, skip_then_retry_until, take_until, Simple},
    text::{self, TextParser},
    Parser,
};

pub type Spanned = (TokenType, Range<usize>);

#[derive(Debug)]
pub struct Lexer;

impl Lexer {
    pub fn parse(source: &str) -> Result<Vec<Spanned>, Vec<Simple<char>>> {
        Self::lex().parse(source)
    }

    #[inline]
    fn lex() -> impl Parser<char, Vec<Spanned>, Error = Simple<char>> {
        // parsers for operators
        let attr = choice((
            just("@@").to(TokenType::BlockAttr),
            just('@').to(TokenType::FieldAttr),
            just('=').to(TokenType::Assign),
            just('?').to(TokenType::Optional),
            just('.').to(TokenType::Dot),
            // parsers for control characters (delimiters, semicolons, etc.)
            just('(').to(TokenType::OpenParen),
            just(')').to(TokenType::CloseParen),
            just('[').to(TokenType::OpenSquare),
            just(']').to(TokenType::CloseSquare),
            just('{').to(TokenType::OpenCurly),
            just('}').to(TokenType::CloseCurly),
            just(':').to(TokenType::Colon),
            just(',').to(TokenType::Comma),
        ));

        // A parser for numbers
        let num = text::int(10)
            .chain::<char, _, _>(just('.').chain(text::digits(10)).or_not().flatten())
            .collect::<String>()
            .map(|x| TokenType::Num(x.parse().unwrap()));

        // A parser for strings
        let string = just('"')
            .ignore_then(filter(|c| *c != '"').repeated())
            .then_ignore(just('"'))
            .collect()
            .map(TokenType::Str);

        // A parser for identifiers and keywords
        let ident = text::ident().map(|ident: String| match ident.as_str() {
            // keywords
            "datasource" => TokenType::DataSource,
            "generator" => TokenType::Generator,
            "model" => TokenType::Model,
            "enum" => TokenType::Enum,
            // datatypes
            "String" => TokenType::String,
            "Int" => TokenType::Int,
            "DateTime" => TokenType::DateTime,
            // everything else is user defined
            "true" => TokenType::Bool(true),
            "false" => TokenType::Bool(false),
            _ => TokenType::Id(ident),
        });

        // A parser for linewise comment
        let comment = just("//").then(take_until(just('\n'))).padded();

        // A single token can be one of the above
        let token = choice((attr, string, num, ident)).recover_with(skip_then_retry_until([]));

        token
            .padded_by(comment.repeated())
            .map_with_span(|ty, range| (ty, range))
            .padded()
            .repeated()
    }
}
