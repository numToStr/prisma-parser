mod token;
pub use token::*;

use std::ops::Range;

use chumsky::{
    prelude::{filter, just, one_of, skip_then_retry_until, take_until, Simple},
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
        // A parser for @@
        let attr = just("@@").to(Token::Attr);

        // A parser for operators
        let operator = one_of(".=?@").map(Token::Op);

        // A parser for control characters (delimiters, semicolons, etc.)
        let ctrl = one_of("()[]{}:,").map(Token::Ctrl);

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
            _ => Token::Ident(ident),
        });

        // A parser for linewise comment
        let comment = just("//").then(take_until(just('\n'))).padded();

        // A single token can be one of the above
        let token = ctrl
            .or(string)
            .or(num)
            .or(attr) // NOTE: attr should come before operator
            .or(operator)
            .or(ident)
            .recover_with(skip_then_retry_until([]));

        token
            .padded_by(comment.repeated())
            .map_with_span(|ty, range| Span { ty, range })
            .padded()
            .repeated()
    }
}
