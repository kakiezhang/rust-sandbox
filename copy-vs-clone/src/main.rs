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

// -----------------

#[derive(Debug, Clone)]
struct School {
    id: u32,
    name: String,
    grade: Grade,
}

#[derive(Debug, Clone, Copy)]
struct Grade {
    id: u32,
    class: u8,
}

fn test_school() {
    let s = School {
        id: 1,
        name: "fudan".to_string(),
        grade: Grade { id: 3, class: 8 },
    };

    let mut s1 = s.clone();
    s1.name = "jiaoda".to_string();
    s1.grade = Grade { id: 4, class: 5 };

    println!("s: {:?}, {:p}, {:?}", s, s.name.as_str(), s.name.as_ptr());
    println!(
        "s1: {:?}, {:p}, {:?}",
        s1,
        s1.name.as_str(),
        s1.name.as_ptr()
    );
}

fn test_school_1() {
    let s = School {
        id: 1,
        name: "fudan".to_string(),
        grade: Grade { id: 3, class: 8 },
    };
    println!("s: {:?}, {:p}, {:?}", s, s.name.as_str(), s.name.as_ptr());

    let mut s1 = s;
    s1.name = "jiaoda".to_string();
    s1.grade = Grade { id: 4, class: 5 };

    println!(
        "s1: {:?}, {:p}, {:?}",
        s1,
        s1.name.as_str(),
        s1.name.as_ptr()
    );
}

fn test_grade() {
    let g = Grade { id: 1, class: 2 };
    let g1 = g;
    println!("g: {:?}, {:p}", g, &g);
    println!("g1: {:?}, {:p}", g1, &g1);
}

#[derive(Debug)]
struct Fruit {
    name: String, // String doesn't imple Copy trait
}

fn test_fruit() {
    let f = Fruit {
        name: "apple".to_string(),
    };
    println!("f: {:?}, {:p}", f, &f);
    let f1 = f;
    println!("f1: {:?}, {:p}", f1, &f1);
}

fn main() {
    // test_school();
    // test_school_1();
    // test_grade();
    test_fruit();
}
