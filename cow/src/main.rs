use std::borrow::Borrow;

fn main() {
    let s = "hello".to_owned();
    let s1: &String = s.borrow();
    let s2: &str = s.borrow();
    println!("s1: {:?}, {:p}", s1, s1);
    println!("s2: {:?}, {:p}", s2, s2);
}
