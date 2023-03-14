use std::borrow::{Borrow, Cow};

fn main() {
    {
        // let s = "hello".to_owned();
        // let s1: &String = s.borrow();
        // let s2: &str = s.borrow();
        // println!("s1: {:?}, {:p}", s1, s1);
        // println!("s2: {:?}, {:p}", s2, s2);
    }

    {
        println!("{:?}", get_owned_string());
    }

    {
        let i1 = foo(10);
        println!("i1: {:?}", i1);

        let i4: &str = i1.as_ref(); // for borrow conversation
        baz1(i4);
        baz1(i4);

        let i2 = foo(11);
        println!("i2: {:?}", i2);

        let i3: String = i2.into_owned(); // for owned conversation
        println!("i3: {:?}", i3);
        baz2(i3);
        // baz2(i3); // use of moved value: `i3`  value used here after move
    }
}

fn get_owned_string() -> String {
    "hello".to_owned()
}

// compile failed!
// fn get_borrowed_string() -> &'static str {
//     let s = "hello".to_owned();
//     s.borrow()
// }

fn foo(i: i32) -> Cow<'static, str> {
    match i % 10 {
        0 => "it's an tener.".into(),
        x => format!("it's not an tener, mod is {}", &x).into(),
    }
}

fn baz1(i: &str) {
    println!("baz1 i: {:?}", i);
}

fn baz2(i: String) {
    println!("baz2 i: {:?}", i);
}
