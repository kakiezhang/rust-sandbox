fn main() {
    let c = Cat {
        name: String::from("miao"),
        typ: animal::Cat,
    };
    println!("{:?}", c.eat());

    take_shower(&c);
}

#[derive(Debug)]
enum animal {
    Cat,
    Dog,
}

trait Action {
    fn eat(&self) -> (String, String);
    fn shower(&self);
}

#[derive(Debug)]
struct Cat {
    name: String,
    typ: animal,
}

impl Action for Cat {
    fn eat(&self) -> (String, String) {
        (self.name.clone(), "fish".into())
    }

    fn shower(&self) {
        println!("{:?} {:?} swim well", self.typ, self.name);
    }
}

fn take_shower(a: &impl Action) {
    a.shower();
}
