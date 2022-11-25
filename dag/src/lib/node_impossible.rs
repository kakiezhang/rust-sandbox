#[derive(Debug)]
struct Node<'a> {
    name: String,
    next: Option<&'a mut Node<'a>>,
}

impl<'a> Node<'a> {
    fn new(name: &str) -> Node {
        return Node {
            name: name.into(),
            next: None,
        };
    }

    fn update_next(&mut self, next: &'a mut Node<'a>) {
        self.next = Some(next);
    }

    // fn get_next(&self) -> &mut Node {
    //     self.next
    // }
}

pub fn test() {
    let mut a = Node::new("A");
    let mut b = Node::new("B");
    let mut c = Node::new("C");

    b.update_next(&mut a);
    println!("b: {:?}, {:p}", b, &b);

    c.update_next(&mut b);
    println!("c: {:?}, {:p}", c, &c);

    let cb = c.next.unwrap();
    println!("cb: {:?}, {:p}", cb, &cb);

    // let mut d = Node::new("D");
    // d.update_next(&mut b); // cannot borrow `b` as mutable more than once at a time  first borrow later used at line 32
    // println!("d: {:?}, {:p}", d, &d);
}
