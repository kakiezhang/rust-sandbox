fn main() {
    // foo1();
    foo2();
}

fn foo1() {
    {
        // let x = Some(42);
        let x: Option<i32> = None;
        let z = x.map_or(Some(100), |y| Some(y + 1));
        println!("z: {:?}", z);
    }

    {
        let a = Some("hello");
        let c = a.map_or(100, |y| y.len());
        println!("c: {:?}", c);
    }
}

fn foo2() {
    // let x = Some("world");
    let x: Option<&str> = None;
    let t = x.map_or_else(|| x.unwrap_or("a") > "g", |y| y > "w");
    println!("t: {:?}", t);
}
