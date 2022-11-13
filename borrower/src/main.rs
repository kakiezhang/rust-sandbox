fn main() {
    // vec data is on the heap
    // vs (on the stack) point to vec data, and own vec data
    // vs1 (on the stack) is a copy of &vs
    let vs = vec![56, 41, 23, 38, 1, 83];
    let vs1 = &vs;
    println!(
        "vs1 address: {:p}, &vs address: {:p} | {:p}, {:p}, {:p}",
        vs1, &vs, &&vs, &vs1, &&vs
    );

    sum(vs1);

    println!(
        "vs1 address: {:p}, &vs address: {:p} | {:p}, {:p}, {:p}",
        vs1, &vs, &&vs, &vs1, &&vs
    );

    println!("items: {:p}, {:p}", &vs[0], &vs[1]);
}

fn sum(data: &Vec<i32>) -> i32 {
    // data addr = vs1 addr
    // &data addr != &vs1 (data is a copy of vs1, so the ref of two are not equal)
    println!("data address: {:p}, &data address: {:p}", data, &data);

    let mut total = 0;
    for (_k, v) in data.iter().enumerate() {
        total += v;
    }
    total
}
