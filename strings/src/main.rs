use colored::Colorize;

fn main() {
    // test0();
    test1();
}

fn color() {
    let a = "Hello world!";
    let b = a;
    // let c = a.as_ref();
    let d: &str = a.as_ref();
    println!("{:?}", d);

    let e = "haha".to_string().blue();
    let f = "hehe".red();
    println!("{}", e);
    println!("{}", f);
}

fn test0() {
    // s type is &str, point to "cat"
    let s = "cat";

    // return value type is String
    let s1 = String::from(s);

    // s2 type is &str
    let s2: &str = s;

    let s3 = &s1;

    println!("s2.eq(&s1) = {:?}", s2.eq(&s1));
    println!("s2.eq(s) = {:?}", s2.eq(s));

    println!("{:p} | {:p}", &s1, &s3);
    println!("{:p} | {:p}", s1.as_ptr(), s3.as_ptr());
}

fn test1() {
    {
        let a: String = "hello".to_string();
        let b = lifetime_1(&a);
    }

    {
        println!("lifetime_2() = {:?}", lifetime_2());
    }

    {
        lifetime_3();
    }
}

fn lifetime_1(a: &String) -> &str {
    let b: &str = &a[0..2];
    println!("b: {:?}", b);
    b
}

fn lifetime_2() -> &'static str {
    let a = "hello";
    let b = &a[1..2];
    b
}

fn lifetime_3() {
    let mut a: &str = "world";
    let b = a.to_owned();
    println!("a: {:?}, b: {:?}", a, b);

    a = &a[1..2];
    println!("a: {:?}, b: {:?}", a, b);
}
