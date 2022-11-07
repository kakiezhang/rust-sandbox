fn main() {
    let a = "Hello world!";
    let b = a;
    // let c = a.as_ref();
    let d: &str = a.as_ref();
    println!("{:?}", d);
}
