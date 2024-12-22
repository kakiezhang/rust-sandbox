use enum_iterator::{all, cardinality, first, last, next, previous, reverse_all, Sequence};

#[derive(Debug, PartialEq, Sequence)]
enum Greeting {
    Hello(u8),
    Hi(u8),
}

struct Hello {
    id: u8,
}

struct Hi {
    id: u8,
}

fn test0() {
    for i in all::<Greeting>().collect::<Vec<_>>() {
        println!("i: {:?}", i);
    }
}

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter, Debug, PartialEq)]
enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST,
}

fn test1() {
    // for i in Direction::
}

fn main() {
    test0()
}
