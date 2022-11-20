use std::rc::Rc;

#[derive(Debug)]
struct Node {
    name: String,
    next: Option<Rc<Node>>,
}

trait Action {
    fn new(_: &str) -> Node;
    fn update_next(&mut self, _: Rc<Node>);
    fn get_next(&self) -> Rc<Node>;
}

impl Action for Node {
    fn new(name: &str) -> Node {
        return Node {
            name: name.into(),
            next: None,
        };
    }
    fn update_next(&mut self, next: Rc<Node>) {
        self.next = Some(next);
    }
    fn get_next(&self) -> Rc<Node> {
        self.next.as_ref().unwrap().clone()
    }
}

fn main() {
    {
        let a: Node = Node::new("A");
        println!("a: {:?} {:p}", a, &a);

        let mut b = Node::new("B");
        let mut c = Node::new("C");
        b.update_next(Rc::new(a));

        let bb = b.get_next();
        println!("bb: {:?} {:p}", bb, &bb);

        c.update_next(Rc::new(b));

        let mut d = Node::new("D");

        d.update_next(bb);

        println!("d: {:?}", d);
        println!("c: {:?}", c);

        // cannot borrow data in an `Rc` as mutable
        let e = Node::new("E");
        let db = d.get_next();
        // Rc is a readonly refence, you cannot get the mutable borrow of the inside node to
        // modify itself
        db.update_next(Rc::new(e));
    }

    // {
    //     let a = Rc::new(8);
    //     let b = a.clone();
    //     let c = a.clone();
    //     // Rc object address are the same
    //     // a, b, c ref address varies
    //     println!("{:?}, {:p}, {:p}", a, a, &a);
    //     println!("{:?}, {:p}, {:p}", b, b, &b);
    //     println!("{:?}, {:p}, {:p}", c, c, &c);
    // }
}
