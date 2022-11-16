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
        self.next.as_ref().unwrap().clone()
    }
}

fn main() {
    let mut a: Node = Node::new("A");
    let b = Node::new("B");
    a.update_next(b);
    println!("{:?}", a);
}
