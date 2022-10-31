fn main() {
    println!("apply(2, square) = {:?}", apply(2, square));
    println!("apply(2, cube) = {:?}", apply(2, cube));
}

fn apply(x: u32, f: fn(u32) -> u32) -> u32 {
    f(x)
}

fn square(x: u32) -> u32 {
    x * x
}

fn cube(x: u32) -> u32{
    x * x*x
}
