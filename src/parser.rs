
use pair::Pair;

pub struct Parser {
    position: u64,
    line: u32,
    column: u32,
    pairs: Pair,
}

