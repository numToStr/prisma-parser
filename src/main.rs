use std::fs::read_to_string;

use pp::Lexer;

fn main() {
    let prisma = read_to_string("src/corpus/test.prisma").unwrap();
    let tokens = Lexer::parse(&prisma);
    dbg!(tokens.unwrap());
    // println!("Hello, world!");
}
