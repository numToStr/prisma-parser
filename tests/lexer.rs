use pp::*;

#[test]
fn lex_datsource() {
    assert_eq!(
        Lexer::parse("datasource db {}").unwrap(),
        [
            Span {
                ty: Token::DataSource,
                range: 0..10
            },
            Span {
                ty: Token::Id("db".into()),
                range: 11..13
            },
            Span {
                ty: Token::LeftCurly,
                range: 14..15
            },
            Span {
                ty: Token::RightCurly,
                range: 15..16
            }
        ]
    );
}

#[test]
fn lex_model() {
    assert_eq!(
        Lexer::parse(
            r#"
                model User {
                    name String @db.Varchar(200)
                }
            "#
        )
        .unwrap(),
        [
            Span {
                ty: Token::Model,
                range: 17..22
            },
            Span {
                ty: Token::Id("User".into()),
                range: 23..27
            },
            Span {
                ty: Token::LeftCurly,
                range: 28..29
            },
            Span {
                ty: Token::Id("name".into()),
                range: 50..54
            },
            Span {
                ty: Token::String,
                range: 55..61
            },
            Span {
                ty: Token::Prop,
                range: 62..63
            },
            Span {
                ty: Token::Id("db".into()),
                range: 63..65
            },
            Span {
                ty: Token::Dot,
                range: 65..66
            },
            Span {
                ty: Token::Id("Varchar".into()),
                range: 66..73
            },
            Span {
                ty: Token::LeftParen,
                range: 73..74
            },
            Span {
                ty: Token::Num("200".into()),
                range: 74..77
            },
            Span {
                ty: Token::RightParen,
                range: 77..78
            },
            Span {
                ty: Token::RightCurly,
                range: 95..96
            },
        ]
    );
}
