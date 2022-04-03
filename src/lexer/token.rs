#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TokenType {
    // Keywords
    DataSource, // datasource
    Generator,  // client
    Model,      // model
    Enum,       // enum

    // Ctrl chars
    OpenParen,   // (
    CloseParen,  // )
    OpenCurly,   // {
    CloseCurly,  // }
    OpenSquare,  // [
    CloseSquare, // ]
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
    Bool(bool), // boolean value

    // Data Types
    DateTime,
    String,
    Int,
}
