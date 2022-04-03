// FIXME: fix these tests
// use pp::*;
//
// #[test]
// fn lex_datsource() {
//     assert_eq!(
//         Lexer::parse("datasource db {}").unwrap(),
//         [
//             Token {
//                 ty: TokenType::DataSource,
//                 range: 0..10
//             },
//             Token {
//                 ty: TokenType::Id("db".into()),
//                 range: 11..13
//             },
//             Token {
//                 ty: TokenType::OpenCurly,
//                 range: 14..15
//             },
//             Token {
//                 ty: TokenType::CloseCurly,
//                 range: 15..16
//             }
//         ]
//     );
// }
//
// #[test]
// fn lex_model() {
//     assert_eq!(
//         Lexer::parse(
//             r#"
//                 model User {
//                     name String @db.Varchar(200)
//                 }
//             "#
//         )
//         .unwrap(),
//         [
//             Token {
//                 ty: TokenType::Model,
//                 range: 17..22
//             },
//             Token {
//                 ty: TokenType::Id("User".into()),
//                 range: 23..27
//             },
//             Token {
//                 ty: TokenType::OpenCurly,
//                 range: 28..29
//             },
//             Token {
//                 ty: TokenType::Id("name".into()),
//                 range: 50..54
//             },
//             Token {
//                 ty: TokenType::String,
//                 range: 55..61
//             },
//             Token {
//                 ty: TokenType::Prop,
//                 range: 62..63
//             },
//             Token {
//                 ty: TokenType::Id("db".into()),
//                 range: 63..65
//             },
//             Token {
//                 ty: TokenType::Dot,
//                 range: 65..66
//             },
//             Token {
//                 ty: TokenType::Id("Varchar".into()),
//                 range: 66..73
//             },
//             Token {
//                 ty: TokenType::OpenParen,
//                 range: 73..74
//             },
//             Token {
//                 ty: TokenType::Num("200".into()),
//                 range: 74..77
//             },
//             Token {
//                 ty: TokenType::CloseParen,
//                 range: 77..78
//             },
//             Token {
//                 ty: TokenType::CloseCurly,
//                 range: 95..96
//             },
//         ]
//     );
// }
