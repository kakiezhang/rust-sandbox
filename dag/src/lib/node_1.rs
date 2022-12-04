#[derive(Debug)]
struct Node<'a, 'b> {
    name: String,
    next: Option<&'a mut &'b Node<'a, 'b>>,
}

impl<'a, 'b> Node<'a, 'b> {
    fn new(name: &str) -> Node {
        return Node {
            name: name.into(),
            next: None,
        };
    }

    fn update_next(&mut self, next: &'a mut &'b Node<'a, 'b>) {
        // println!("next: {:?}, {:p}", next, next);
        self.next = Some(next);
    }
}

pub fn test() {
    let a = Node::new("A");
    let mut b = Node::new("B");
    let mut c = Node::new("C");

    let mut bb = &a;
    b.update_next(&mut bb);
    println!("b: {:?}, {:p}", b, &b);

    let mut cb0 = &b;
    println!("cb0: {:?}, {:p}", cb0, &cb0);
    c.update_next(&mut cb0);
    println!("c: {:?}, {:p}", c, &c);

    let cb1 = c.next.unwrap(); // cb1 == cb0
    println!("cb1: {:?}, {:p}", cb1, cb1);

    let mut d = Node::new("D");
    let mut db0 = &b;
    d.update_next(&mut db0);
    println!("d: {:?}, {:p}", d, &d);

    let db1 = d.next.unwrap(); // db1 != cb1
    println!("db1: {:?}, {:p}", db1, db1);
}
