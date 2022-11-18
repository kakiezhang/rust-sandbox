use std::rc::Rc;

#[derive(Debug)]
struct Node {
    name: String,
}

fn main() {
    // {
    //     let x = Rc::new(10);
    //     let _y = x.clone();
    //     // let _y = Rc::clone(&x); // same as the previous line
    //     let y = *x;
    //     println!("y0: {:?}", y);
    //     println!("y1: {:?}", y);
    //     assert_eq!(y, 10);
    // }

    // {
    //     let x = Rc::new(4);
    //     let _y = Rc::clone(&x);
    //     assert_eq!(*x, 4);
    //     assert_eq!(*Rc::try_unwrap(x).unwrap_err(), 4);
    //     // assert_eq!(*x, 4);
    // }

    {
        let a = Node {
            name: "A".to_string(),
        };
        let x = Rc::new(a);
        let y = x.clone();

        println!("y0: {:?}", *y);
        println!("y1: {:?}", *y);

        println!("{:?}", y.name.eq("A"));
        println!("{:?}", y.name.eq("A"));
    }
}
