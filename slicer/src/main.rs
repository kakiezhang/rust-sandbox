fn main() {
    let arr = [11, 12, 13, 14, 15]; // on the stack
    let vec = vec![11, 12, 13, 14, 15]; // on the heap

    let as1 = &arr[1..3]; // on the stack
    let vs1 = &vec[1..3]; // on the stack

    assert!(as1 == vs1);
    assert!(as1 == vec![12, 13]);
    assert!(vs1 == [12, 13]);

    arr_to_slice(&[21, 22, 23]);

    let v1 = vec![22, 23];
    arr_to_slice(&v1[..]);
}

fn arr_to_slice<T>(x: &[T])
where
    T: std::fmt::Debug,
{
    println!("x: {:?}", x);
}
