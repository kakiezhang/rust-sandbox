use std::cmp::Ordering;
use std::cmp::{PartialEq, PartialOrd};

#[derive(Debug)]
struct Threes(Box<[[Card; 3]]>);

#[derive(Debug)]
struct ThreeWithOnes(Box<[([Card; 3], [Card; 1])]>);

#[derive(Debug)]
struct ThreeWithTwos(Box<[([Card; 3], [Card; 2])]>);

#[derive(Debug)]
struct Chain(Box<[Card]>);

#[derive(Debug)]
struct Pairs(Box<[[Card; 2]]>);

trait Lower {
    fn low(&self) -> bool;
}

impl Lower for Chain {
    fn low(&self) -> bool {
        self.0.len() >= 5
    }
}

impl Lower for Pairs {
    fn low(&self) -> bool {
        self.0.len() >= 3
    }
}

trait Upper {
    fn up(&self) -> u8;
}

#[derive(Debug)]
struct Card {
    point: Point,
    color: Color,
}

impl Card {
    fn unwrap_point(&self) -> u8 {
        match self.point {
            Point::King(x) => x,
            Point::Queen(x) => x,
            Point::Jack(x) => x,
            Point::Ten(x) => x,
        }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.unwrap_point() == other.unwrap_point()
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        None
    }

    fn gt(&self, other: &Self) -> bool {
        self.unwrap_point() > other.unwrap_point()
    }

    fn ge(&self, other: &Self) -> bool {
        self.unwrap_point() >= other.unwrap_point()
    }

    fn le(&self, other: &Self) -> bool {
        !self.gt(other)
    }

    fn lt(&self, other: &Self) -> bool {
        !self.ge(other)
    }
}

#[derive(Debug)]
enum Point {
    King(King),
    Queen(Queen),
    Jack(Jack),
    Ten(Ten),
}

type King = u8;

type Queen = u8;

type Jack = u8;

type Ten = u8;

#[derive(Debug)]
enum Color {
    Spades,
    Plum,
    Square,
    Hearts,
}

fn main() {
    let t1 = Card {
        point: Point::Ten(10),
        color: Color::Spades,
    };

    let t2 = Card {
        point: Point::King(13),
        color: Color::Plum,
    };

    // let t3 = Card {
    //     point: Point::Jack(11),
    //     color: Color::Square,
    // };

    println!("{:?}", t2 > t1);

    let pc = Chain(Box::new([t1, t2]));
    println!("{:?}", pc);

    println!("pc.low(): {:?}", pc.low());

    // println!("{:?}", t2.point);

    // let a: Ten = 10;
    // let b: Jack = 11;
    // println!("{:?}", b > a);
}
