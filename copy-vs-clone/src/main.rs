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

#[derive(Debug, Clone)]
struct Supermarket<'a> {
    goods: Goods<'a>,
}

#[derive(Debug, Clone)]
struct Goods<'a> {
    v: &'a Vec<&'a str>,
}

#[test]
fn test_goods() {
    let mut goods = &mut vec!["cup".into(), "fish".into(), "meat".into(), "eggs".into()];

    let a = Supermarket {
        goods: Goods { v: &mut goods },
    };

    let b = a.clone();

    if let Some(elem) = goods.get_mut(0) {
        *elem = "water".into();
    }

    // *goods.get_mut(0).unwrap() = "water".into();

    // (*b.goods.v)[0] = "water".into();

    // b.goods.v[0] = "water".into();

    println!("a: {:?}", a);
    println!("b: {:?}", b);

    assert_eq!(a.goods.v, b.goods.v);
}

fn main() {}
