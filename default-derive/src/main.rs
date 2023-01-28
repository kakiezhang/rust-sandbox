#[derive(Debug, Default)]
struct Animal {
    id: ID,
}

#[derive(Debug)]
struct ID {
    v: u8,
}

impl Default for ID {
    fn default() -> Self {
        ID { v: 20 }
    }
}

fn main() {
    let a = Animal::default();
    println!("a: {:?}", a);
}
