fn main() {
    // common usage
    let c = Cat {
        name: String::from("miao"),
        typ: AnimalType::Cat,
    };
    println!("{:?}", c.eat());

    // impl as param
    take_shower(&c);

    let c1 = Cat::fromer(AnimalType::Cat);
    println!("{:?}", c1.eat());
}

#[derive(Debug)]
enum AnimalType {
    Cat,
    Dog,
}

trait Animal {
    fn eat(&self) -> (String, String);
    fn shower(&self);
}

#[derive(Debug)]
struct Cat {
    name: String,
    typ: AnimalType,
}

impl Animal for Cat {
    fn eat(&self) -> (String, String) {
        (self.name.clone(), "fish".into())
    }

    fn shower(&self) {
        println!("{:?} {:?} swim well", self.typ, self.name);
    }
}

fn take_shower(a: &impl Animal) {
    a.shower();
}

trait Fromer<T, U> {
    fn fromer(_: T) -> U;
}

impl Fromer<AnimalType, Box<dyn Animal>> for Cat {
    fn fromer(_: AnimalType) -> Box<(dyn Animal + 'static)> {
        Box::new(Cat {
            name: "kitty".to_string(),
            typ: AnimalType::Cat,
        })
    }
}
