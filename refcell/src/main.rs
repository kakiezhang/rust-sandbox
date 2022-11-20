use std::cell::RefCell;

fn main() {
    // use RefCell to change the v which is immutable borrow that cannot be changed by default
    // RefCell bypass the compiler checking, change the v at runtime
    // curly braces is needed because mutable and immutable borrows shouldn't be at the same scope
    let v = RefCell::new(1);
    {
        let mut p = v.borrow_mut();
        *p += 1;
        println!("p: {:?}", p);
    }
    println!("v.borrow() = {:?}", v.borrow());
}
