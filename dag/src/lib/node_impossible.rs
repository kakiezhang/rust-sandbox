#[derive(Debug)]
struct Node {
    name: String,
    next: Option<*mut Node>,
}

impl Node {
    fn new(name: &str) -> Node {
        return Node {
            name: name.into(),
            next: None,
        };
    }
    fn update_next(&mut self, next: *mut Node) {
        self.next = Some(next);
    }
    fn get_next(&self) -> *mut Node {
        self.next.unwrap()
    }
}

pub fn test() {
    let mut a = Node::new("A");
    let mut b = Node::new("B");
    let mut c = Node::new("C");

    b.update_next(&mut a);
    c.update_next(&mut b);

    println!("c: {:?}, {:p}", c, &c);
    println!("b: {:?}, {:p}", b, &b);

    let mut d = Node::new("D");
    d.update_next(&mut b);
    println!("d: {:?}, {:p}", d, &d);

    let mut e = Node::new("E");
    println!("e: {:?}, {:p}", e, &e);
    let db = d.get_next();
    unsafe { (*db).update_next(&mut e) };
    // unsafe {
    //     let h = &mut (*db);
    //     h.update_next(&mut e)
    // };
    println!("d: {:?}, {:p}", d, &d);
    println!(
        "db: {:?}, {:p}",
        unsafe { (*d.next.unwrap()).name.clone() },
        &(d.next)
    );
}
