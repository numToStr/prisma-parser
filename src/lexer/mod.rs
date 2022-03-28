mod token;
pub use token::*;

use std::ops::Range;

use chumsky::{
    prelude::{filter, just, skip_then_retry_until, take_until, Simple},
    text::{self, TextParser},
    Parser,
};

type Err = Simple<char>;

#[derive(Debug, PartialEq)]
pub struct Span {
    pub ty: Token,
    pub range: Range<usize>,
}

#[derive(Debug)]
pub struct Lexer;

impl Lexer {
    pub fn parse(source: &str) -> Result<Vec<Span>, Vec<Err>> {
        Self::lex().parse(source)
    }

    fn lex() -> impl Parser<char, Vec<Span>, Error = Err> {
        // parsers for operators
        let attr = just("@@")
            .to(Token::Attr)
            .or(just('@').to(Token::Prop))
            .or(just('=').to(Token::Assign))
            .or(just('?').to(Token::Optional))
            .or(just('.').to(Token::Dot))
            // parsers for control characters (delimiters, semicolons, etc.)
            .or(just('(').to(Token::LeftParen))
            .or(just(')').to(Token::RightParen))
            .or(just('[').to(Token::LeftSquare))
            .or(just(']').to(Token::RightSquare))
            .or(just('{').to(Token::LeftCurly))
            .or(just('}').to(Token::RightCurly))
            .or(just(':').to(Token::Colon))
            .or(just(',').to(Token::Comma));

        // A parser for numbers
        let num = text::int(10)
            .chain::<char, _, _>(just('.').chain(text::digits(10)).or_not().flatten())
            .collect::<String>()
            .map(Token::Num);

        // A parser for strings
        let string = just('"')
            .ignore_then(filter(|c| *c != '"').repeated())
            .then_ignore(just('"'))
            .collect()
            .map(Token::Str);

        // A parser for identifiers and keywords
        let ident = text::ident().map(|ident: String| match ident.as_str() {
            // keywords
            "datasource" => Token::DataSource,
            "generator" => Token::Generator,
            "model" => Token::Model,
            "enum" => Token::Enum,
            // datatypes
            "String" => Token::String,
            "Int" => Token::Int,
            "DateTime" => Token::DateTime,
            // everything else is user defined
            _ => Token::Id(ident),
        });

        // A parser for linewise comment
        let comment = just("//").then(take_until(just('\n'))).padded();

        // A single token can be one of the above
        let token = attr
            .or(string)
            .or(num)
            .or(ident)
            .recover_with(skip_then_retry_until([]));

        token
            .padded_by(comment.repeated())
            .map_with_span(|ty, range| Span { ty, range })
            .padded()
            .repeated()
    }
}
