#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Token {
    // Keywords
    DataSource,    // datasource
    Generator,     // client
    Model,         // model
    Enum,          // enum
    Ctrl(char),    // (){}()
    Op(char),      // =?@
    Ident(String), // like source/generator/model name
    Str(String),   // string value
    Num(String),   // number value
    Attr,          // @@

    // Data Types
    DateTime,
    String,
    Int,
}
