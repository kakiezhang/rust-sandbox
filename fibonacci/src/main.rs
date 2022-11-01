fn main() {
    let n = 6;
    fib_loop(n);
    fib_while(n);
    fib_for(n);

    // close interval at the end of array
    let arr = [54, 18, 66, 37, 23, 89];
    for i in &arr[0..=2] {
        println!("{:?}", i);
    }
}

fn fib_loop(end: u32) {
    // use loop
    let mut a = 1;
    let mut b = 1;
    let mut x = 0;

    // c should be read before writen, or compiler will think that c will be overwitten before read
    // one var should be read after being declared
    let mut c = 0;

    loop {
        if x >= end {
            break;
        } else {
            x += 1;
        }

        c = a + b;
        a = b;
        b = c;
    }

    println!("a: {:?}", a);
    println!("b: {:?}", b);
}

fn fib_while(end: u32) {
    // use while
    let mut a = 1;
    let mut b = a;
    let mut x = 0;

    while x < end {
        let c = a + b;
        a = b;
        b = c;
        x += 1;
    }

    println!("a: {:?}", a);
    println!("b: {:?}", b);
}

fn fib_for(end: u32) {
    // use for
    let (mut a, mut b, mut c) = (1, 1, 0);

    for _x in 0..end {
        c = a + b;
        a = b;
        b = c;
    }

    println!("a: {:?}", a);
    println!("b: {:?}", b);
}
