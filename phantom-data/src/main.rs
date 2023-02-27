use std::marker::PhantomData;

fn main() {
    let i1 = Animal::<Insect>::new(1, "a worm".into());
    let m2 = Animal::<Monkey>::new(2, "a monkey".into());

    let m1: Animal<Monkey> = i1.into();

    if m1 > m2 {
        println!("m1 > m2");
    } else {
        println!("m1 <= m2");
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Animal<T> {
    id: u64,
    name: String,
    _tag: PhantomData<T>,
}

impl<T> Animal<T> {
    fn new(id: u64, name: String) -> Self {
        Self {
            id,
            name,
            _tag: PhantomData::default(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Insect;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Monkey;

impl From<Animal<Insect>> for Animal<Monkey> {
    fn from(x: Animal<Insect>) -> Self {
        Self::new(x.id, x.name)
    }
}
