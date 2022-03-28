#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    DataSource, // datasource
    Generator,  // client
    Model,      // model
    Enum,       // enum

    // Ctrl chars
    LeftParen,   // (
    RightParen,  // )
    LeftCurly,   // {
    RightCurly,  // }
    LeftSquare,  // [
    RightSquare, // ]
    Colon,       // :
    Comma,       // ,

    // Operators
    Attr,     // @@
    Prop,     // @
    Assign,   // =
    Optional, // ?
    Dot,      // .

    // literals
    Id(String), // user defined identified (like source/generator/model name, table fields)
    Str(String), // string value
    Num(String), // number value

    // Data Types
    DateTime,
    String,
    Int,
}
