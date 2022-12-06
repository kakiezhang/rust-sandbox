use std::fmt::Debug;

fn main() {
    let c = Cat { name: "Miao" };
    println!("{:?}", c);

    collect0(c);

    let d = Dog { name: "Wang" };
    collect1(d);
}

trait Animal {
    fn eat() -> &'static str;
}

#[derive(Debug)]
struct Cat {
    name: &'static str,
}

impl Animal for Cat {
    fn eat() -> &'static str {
        "fish"
    }
}

#[derive(Debug)]
struct Dog {
    name: &'static str,
}

impl Animal for Dog {
    fn eat() -> &'static str {
        "shit"
    }
}

fn collect0(a: impl Animal + Debug) {
    println!("{:?}", a);
}

fn collect1<T: Animal>(a: T)
where
    T: Debug,
{
    println!("{:?}", a);
}
