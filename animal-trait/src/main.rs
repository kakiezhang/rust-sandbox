use std::fmt::Debug;

fn main() {
    let c = Cat { name: "Miao" };
    println!("{:?}", c);

    collect0(c);

    let d = Dog { name: "Wang" };
    collect1(d);

    let mut vs: Vec<Box<dyn Animal>> = Vec::new();
    let c1 = Cat { name: "Miao_1" };
    let d1 = Dog { name: "Wang_1" };
    vs.push(Box::new(c1));
    vs.push(Box::new(d1));
    collect2::<dyn Animal>(vs);
}

trait Animal {
    fn eat(&self) -> &'static str;
}

#[derive(Debug)]
struct Cat {
    name: &'static str,
}

impl Animal for Cat {
    fn eat(&self) -> &'static str {
        "fish"
    }
}

#[derive(Debug)]
struct Dog {
    name: &'static str,
}

impl Animal for Dog {
    fn eat(&self) -> &'static str {
        "shit"
    }
}

fn collect0(a: impl Animal + Debug) {
    println!("{:?}", a);
}

// collect1 equals collect0
// "impl Animal" equals "<T: Animal>(a: T)"
fn collect1<T: Animal>(a: T)
where
    T: Debug,
{
    println!("{:?}", a);
}

fn collect2<T: Animal + ?Sized>(vs: Vec<Box<dyn Animal>>) {
    for v in vs {
        println!("{:?}", v.eat());
    }
}
