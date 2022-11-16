#[derive(Debug)]
enum Voice {
    A(Haha),
    E(Hehe),
}

#[derive(Debug)]
struct Haha {
    name: String,
}

#[derive(Debug)]
struct Hehe {
    name: String,
}

fn main() {
    let v = Voice::A(Haha {
        name: "haha1".to_string(),
    });
    let a = match v {
        Voice::A(ha) => ha.name,
        Voice::E(he) => he.name,
    };
    println!("a: {:?}", a);
}
