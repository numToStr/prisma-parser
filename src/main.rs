use std::fs::read_to_string;

use pp::Prisma;

fn main() {
    let prisma = read_to_string("src/corpus/smol.prisma").unwrap();
    let parsed = Prisma::parse(&prisma).unwrap();
    dbg!(parsed);
}
