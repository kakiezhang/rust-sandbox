fn main() {
    // if let
    let x = Animal::Drink(10);
    if let Animal::Drink(y) = x {
        println!("y: {:?}", y);
    } else {
        println!("{:?}", "no_0");
    }

    // normal match
    let y = Animal::Eat(food { name: 2 });
    match y {
        Animal::Eat(z) => println!("z: {:?}", z),
        _ => println!("no_1"),
    }
}

#[derive(Debug)]
enum Animal {
    Run(u32),
    Eat(food),
    Drink(u32),
}

#[derive(Debug)]
struct food {
    name: u32,
}
