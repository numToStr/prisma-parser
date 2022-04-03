use std::fs::read_to_string;

use pp::{Lexer, Parser};

fn main() {
    let prisma = read_to_string("src/corpus/smol.prisma").unwrap();
    let tokens = Lexer::parse(&prisma).unwrap();
    // dbg!(&tokens);
    let parsed = Parser::parse(&tokens).unwrap();
    dbg!(parsed);
    // println!("Hello, world!");
}
