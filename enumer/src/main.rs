#[derive(Debug)]
enum Voice {
    HA(Haha),
    HE(Hehe),
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
    let v = Voice::HA(Haha {
        name: "hahaha..".to_string(),
    });
    let a = match v {
        Voice::HA(ha) => ha.name,
        Voice::HE(he) => he.name,
    };
    println!("a: {:?}", a);
}
