fn main() {
    let idx = find_pos(vec![38, 88, 55], 5);
    println!("{}", idx);
}

fn test_sum_1() {
    // let v = 1;
    let s = vec![1, 2];
    let s1 = s.clone();
    println!("{:?}", s);
}

fn sum_1(s: String, a: i32) -> i32 {
    a + 1
}

// fn test_can_copy() {
//     can_copy("abcd");

//     can_copy(1);

//     can_copy(vec![1]);

//     can_copy("a".into());

//     let a: &str = "a";
//     can_copy(a);

//     let mut b: String = "a".to_string();
//     can_copy(b);
// }

fn can_copy<T: Copy>(_t: T) {}

fn find_pos(vs: Vec<i32>, target: i32) -> i32 {
    for v in vs.iter().enumerate() {
        // println!("k: {}, v: {}", v.0, v.1);
        if *v.1 == target {
            return v.0 as i32;
        }
    }
    -1
}
