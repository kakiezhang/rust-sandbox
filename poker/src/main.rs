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

trait Suit {
    fn suit(&self) -> bool;
}

impl Suit for Chain {
    fn suit(&self) -> bool {
        if self.0.len() < 5 {
            return false;
        }

        let mut v1 = vec![Card::default(); self.0.len()];
        v1.clone_from_slice(&self.0);

        // println!("before sort, v1: {:?}", v1);
        v1.sort_by(|x, y| x.partial_cmp(y).unwrap());
        // println!("after sort, v1: {:?}", v1);

        let mut m = v1[0].unwrap_point();
        for x in v1 {
            let xp = x.unwrap_point();
            if m != xp {
                return false;
            }
            m = xp + 1;
        }

        true
    }
}

impl Suit for Pairs {
    fn suit(&self) -> bool {
        self.0.len() >= 3
    }
}

trait Upper {
    fn up(&self) -> u8;
}

#[derive(Debug, Clone)]
struct Card {
    point: Point,
    color: Color,
}

impl Card {
    fn new(p: Point, c: Color) -> Card {
        let tp = match p {
            Point::GoldenJoker(_) => Point::GoldenJoker(100),
            Point::SilverJoker(_) => Point::SilverJoker(90),
            Point::BigTwo(_) => Point::BigTwo(20),
            Point::Ace(_) => Point::Ace(14),
            Point::King(_) => Point::King(13),
            Point::Queen(_) => Point::Queen(12),
            Point::Jack(_) => Point::Jack(11),
            Point::Ten(_) => Point::Ten(10),
            Point::Nine(_) => Point::Nine(9),
            Point::Eight(_) => Point::Eight(8),
            Point::Seven(_) => Point::Seven(7),
            Point::Six(_) => Point::Six(6),
            Point::Five(_) => Point::Five(5),
            Point::Four(_) => Point::Four(4),
            Point::Three(_) => Point::Three(3),
            Point::Null(_) => Point::Null(0),
        };

        Card {
            point: tp,
            color: c,
        }
    }

    fn default() -> Card {
        Card {
            point: Point::Null(0),
            color: Color::Null,
        }
    }

    fn unwrap_point(&self) -> u8 {
        match self.point {
            Point::GoldenJoker(x) => x,
            Point::SilverJoker(x) => x,
            Point::BigTwo(x) => x,
            Point::Ace(x) => x,
            Point::King(x) => x,
            Point::Queen(x) => x,
            Point::Jack(x) => x,
            Point::Ten(x) => x,
            Point::Nine(x) => x,
            Point::Eight(x) => x,
            Point::Seven(x) => x,
            Point::Six(x) => x,
            Point::Five(x) => x,
            Point::Four(x) => x,
            Point::Three(x) => x,
            Point::Null(_) => 0,
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
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.gt(other) {
            Some(Ordering::Greater)
        } else if self.lt(other) {
            Some(Ordering::Less)
        } else {
            Some(Ordering::Equal)
        }
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

#[derive(Debug, Clone)]
enum Point {
    GoldenJoker(GoldenJoker),
    SilverJoker(SilverJoker),
    BigTwo(BigTwo),
    Ace(Ace),
    King(King),
    Queen(Queen),
    Jack(Jack),
    Ten(Ten),
    Nine(Nine),
    Eight(Eight),
    Seven(Seven),
    Six(Six),
    Five(Five),
    Four(Four),
    Three(Three),
    Null(Null),
}

type GoldenJoker = u8;
type SilverJoker = u8;
type BigTwo = u8;
type Ace = u8;
type King = u8;
type Queen = u8;
type Jack = u8;
type Ten = u8;
type Nine = u8;
type Eight = u8;
type Seven = u8;
type Six = u8;
type Five = u8;
type Four = u8;
type Three = u8;

type Null = u8;

#[derive(Debug, Clone)]
enum Color {
    Spades,
    Plum,
    Square,
    Hearts,
    Null,
}

fn main() {
    let t1 = Card::new(Point::Ten(0), Color::Spades);

    let t2 = Card::new(Point::King(0), Color::Plum);

    let t3 = Card::new(Point::Jack(0), Color::Square);

    let t4 = Card::new(Point::Queen(0), Color::Square);

    let t5 = Card::new(Point::Ace(0), Color::Hearts);

    let pc = Chain(Box::new([t1, t2, t3, t4, t5]));

    println!("Chain.suit(): {:?}", pc.suit());

    // let a: Ten = 10;
    // let b: Jack = 11;
    // println!("{:?}", b > a);
}
