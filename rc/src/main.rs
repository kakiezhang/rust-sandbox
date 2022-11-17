use std::rc::Rc;

fn main() {
    let x = Rc::new(10);
    let y = match Rc::try_unwrap(x) {
        Ok(v) => v,
        Err(e) => panic!("ERR: {:?}", e),
    };
    println!("{:?}", y);
}
