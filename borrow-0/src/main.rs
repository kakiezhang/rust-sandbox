fn main() {
    // test_ref_pushed_vec_0();

    // test_ref_pushed_vec_1();

    // test_ref_pushed_vec_2();
}

// fn test_push_local_ref() {
//     let mut vs: Vec<&i32> = Vec::new();
//     push_local_ref(&mut vs);
//     println!("vs: {:?}", vs);
// }

// fn push_local_ref(vs: &mut Vec<&i32>) {
//     let v = 42;
//     vs.push(&v);
// }

// can't borrow &mut (mutable refence) after immutable borrow
fn test_ref_pushed_vec_0() {
    let mut vs = vec![3, 8];
    let vs1 = vec![&vs[0]];
    println!("vs[0] addr: {:p}", &vs[0]);

    // when vs expand length to 100, compiler will create a new heap to copy vs element inside into it
    // and, old heap will be discarded, vs1 couldn't find the right way to point element
    for i in 0..=100 {
        vs.push(i);
    }

    println!("{:p}", &vs1[0]);
}

// can't borrow &mut(mutable refence) twice or more
fn test_ref_pushed_vec_1() {
    let mut vs = vec![1, 2, 3];

    // iter_mut has already use &mut once
    // vs.push use &mut again
    for v in vs.iter_mut() {
        vs.push(*v + 1);
    }

    println!("{:?}", vs);
}

// can't borrow &mut (mutable refence) after immutable borrow
fn test_ref_pushed_vec_2() {
    let mut vs = vec![1, 2, 3];
    let last = vs.last();
    vs.push(4);
    println!("last: {:?}", last);

    // let mut vs = vec![1, 2, 3];
    // let last = vs.last();
    // println!("last: {:?}", last);
    // vs.push(4);

    // let mut vs = vec![1, 2, 3];
    // if let Some(&last) = vs.last() {
    //     vs.push(4);
    //     println!("last: {:?}", last);
    // }
}
