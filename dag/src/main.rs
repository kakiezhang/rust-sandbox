use std::rc::Rc;

#[derive(Debug)]
struct Node {
    name: String,
    next: Option<Rc<Node>>,
}

trait Action {
    fn new(_: &str) -> Node;
    fn update_next(&mut self, _: Node);
    fn get_next(&self) -> Rc<Node>;
}

impl Action for Node {
    fn new(name: &str) -> Node {
        return Node {
            name: name.into(),
            next: None,
        };
    }
    fn update_next(&mut self, next: Node) {
        self.next = Some(Rc::new(next));
    }
    fn get_next(&self) -> Rc<Node> {
        // Rc::clone(self.next.as_ref().unwrap())
        self.next.as_ref().unwrap().clone()
    }
}

fn main() {
    let a: Node = Node::new("A");
    let mut b = Node::new("B");
    let mut c = Node::new("C");
    b.update_next(a);
    c.update_next(b);

    // let mut e = Node::new("E");
    // e.update_next(b); // b has moved, can't be borrowed here

    println!("{:?}", c);

    // println!("{:?}", c.next.unwrap());
    // println!("{:?}", c.next);

    // println!(
    //     "c: {:?}, addr: {:p}",
    //     c.next.as_ref().unwrap(),
    //     c.next.as_ref().unwrap()
    // );

    // let cb1 = Some(c.get_next());
    // println!(
    //     "cb1: {:?}, addr: {:p}",
    //     cb1.as_ref().unwrap(),
    //     cb1.as_ref().unwrap()
    // );

    let mut d = Node::new("D");

    let cb4 = c.get_next();
    let cb2: Node = match Rc::try_unwrap(cb4) {
        Ok(v) => {
            // println!("v: {:?} addr: {:p}", v, &v);
            v
        }
        Err(e) => panic!("err: {:?} hhhhh", e),
    };

    // let cb3 = Rc::try_unwrap(c.get_next()).unwrap();

    // d.update_next(cb3);

    println!("{:?}", d);
}
