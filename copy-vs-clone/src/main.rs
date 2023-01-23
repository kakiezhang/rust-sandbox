#[derive(Debug, Clone, Copy)]
struct Animal {
    dog: Dog,
}

#[derive(Debug, Clone, Copy)]
struct Dog {
    age: i8,
}

#[test]
fn test_copy() {
    // Copy must implement Clone
    let a = Animal {
        dog: Dog { age: 8 },
    };

    // dog copied too
    let mut b = a;
    b.dog.age = 18;

    assert_ne!(a.dog.age, b.dog.age);
}

#[derive(Debug, Clone)]
struct Food {
    milk: Milk,
}

#[derive(Debug, Clone)]
struct Milk {
    weight: i64,
}

#[test]
fn test_clone() {
    let a = Food {
        milk: Milk { weight: 10 },
    };

    let mut b = a.clone();
    b.milk.weight = 20;

    println!("a: {:?}", a);
    println!("b: {:?}", b);

    assert_ne!(a.milk.weight, b.milk.weight);
}

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Supermarket<'a> {
    goods: Goods<'a>,
}

#[derive(Debug)]
struct Goods<'a> {
    v: Rc<RefCell<Vec<&'a str>>>,
}

#[test]
fn test_goods_1() {
    let g0: &str = "cup";
    let g1: &str = "fish";
    let g2: &str = "meat";
    let g3: &str = "eggs";

    let goods = Rc::new(RefCell::new(vec![g0, g1, g2, g3]));
    let goods_1 = goods.clone();

    let a = Supermarket {
        goods: Goods { v: goods },
    };

    let b = Supermarket {
        goods: Goods { v: goods_1 },
    };

    *a.goods.v.borrow_mut().get_mut(0).unwrap() = "water";
    *b.goods.v.borrow_mut().get_mut(3).unwrap() = "milk";

    println!("a: {:?}", a);
    println!("b: {:?}", b);

    assert_eq!(a.goods.v, b.goods.v);
}

// #[test]
// fn test_goods_2() {
//     let g0: &str = "cup";
//     let g1: &str = "fish";
//     let g2: &str = "meat";
//     let g3: &str = "eggs";

//     let goods = vec![g0, g1, g2, g3];

//     let mut a = Supermarket {
//         goods: Goods { v: &goods },
//     };

//     let b = Supermarket {
//         goods: Goods { v: &goods },
//     };

//     *a.goods.v.get_mut(0).unwrap() = "water";

//     // let g4: &str = "water";
//     // if let Some(elem) = a.goods.v.get_mut(0) {
//     //     *elem = g4;
//     // }

//     println!("a: {:?}", a);
//     println!("b: {:?}", b);

//     assert_eq!(a.goods.v, b.goods.v);
// }

fn main() {}
