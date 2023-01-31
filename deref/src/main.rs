use std::ops::{Deref, DerefMut};

#[derive(Debug, Default)]
struct Animal<'a> {
    food: Food<'a>,
    age: u8,
}

#[derive(Debug)]
struct Food<'a> {
    v: &'a str,
    exp: u8,
}

impl<'a> Deref for Animal<'a> {
    type Target = Food<'a>;

    fn deref(&self) -> &Self::Target {
        &self.food
    }
}

impl<'a> DerefMut for Animal<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.food
    }
}

impl<'a> Animal<'a> {
    fn get_age(&self) -> u8 {
        self.age
    }
}

impl<'a> Default for Food<'a> {
    fn default() -> Self {
        Self {
            v: "water",
            exp: 15,
        }
    }
}

impl<'a> Food<'a> {
    fn set_exp(&mut self, v: u8) {
        self.exp = v;
    }
}

fn main() {
    {
        let mut a = Animal::default();
        println!("{:?}", a);

        // this two equal
        println!("{:?}", a.deref());
        println!("{:?}", *a);

        println!("{:?}", a.get_age());

        *a = Food { v: "meat", exp: 8 };
        println!("{:?}", *a);

        (*a).set_exp(18);
        println!("{:?}", *a);

        // animal can direct call food method when deref impled
        a.set_exp(2);
        println!("{:?}", *a);
    }

    {
        let mut a = 10;
        println!("a: {:?}", a);

        let b = &mut a;
        println!("b: {:?}", b);

        let c = *b;
        println!("c: {:?}", c);

        *b = 11;
        println!("a: {:?}", a);
    }
}
