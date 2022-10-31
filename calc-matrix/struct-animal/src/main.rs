fn main() {
    // if let
    let x = action::eat;
    if let action::run = x {
        println!("{:?}", x);
    } else {
        println!("{:?}", "hi");
    }

    // normal match
    match x {
        action::run => println!("yes"),
        _ => println!("no"),
    }
}

#[derive(Debug)]
enum action {
    run,
    eat,
    drink,
}

// #[derive(Debug)]
// struct cat {
//     field: Type
// }

// #[derive(Debug)]
// struct animal {
//     id: u8,
//     name: String,
// }
