fn main() {
    test_sum_1();

    test_find_pos();
}

fn test_sum_1() {
    let s = vec![1, 2];
    let s1 = s;

    // borrow of moved value: `s`  value borrowed here after move
    // println!("{:?}", s);

    sum_1(s1);

    // borrow of moved value: `s1`  value borrowed here after move
    // println!("{:?}", s1);
}

fn sum_1(_vs: Vec<i32>) -> i32 {
    1
}

fn test_can_copy() {
    can_copy::<i8>();
    can_copy::<u64>();
    can_copy::<usize>();

    // function is a pointer
    can_copy::<fn()>();

    // immutable reference is Copy
    can_copy::<&Vec<i32>>();
    can_copy::<&String>();
    can_copy::<&str>();

    // raw pointer is Copy
    can_copy::<*const String>();
    can_copy::<*mut String>();

    // array & tuple with values which is Copy is Copy
    can_copy::<[&String; 2]>();
    can_copy::<(&str, i32)>();
}

fn test_cannot_copy() {
    can_copy::<String>();
    can_copy::<str>();
    can_copy::<Vec<i32>>();

    // mutable reference is not Copy
    can_copy::<&mut String>();

    can_copy::<[String; 3]>();
    can_copy::<(Vec<i32>, i32)>();
}

fn can_copy<T: Copy>() {}

fn test_find_pos() {
    let data = vec![38, 88, 55];
    let v = 55;
    if let Some(idx) = find_pos(data, v) {
        println!("{}", idx);
    }
}

fn find_pos(vs: Vec<i32>, target: i32) -> Option<i32> {
    for v in vs.iter().enumerate() {
        // println!("k: {}, v: {}", v.0, v.1);
        if *v.1 == target {
            return Some(v.0 as i32);
        }
    }
    None
}
