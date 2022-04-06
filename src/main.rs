use std::fs::read_to_string;

use prisma_parser::Prisma;

fn main() {
    let prisma = read_to_string("src/fixtures/smol.prisma").unwrap();
    let parsed = Prisma::parse(&prisma).unwrap();
    dbg!(parsed);
}
