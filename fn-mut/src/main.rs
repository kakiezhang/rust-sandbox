fn foo<F: FnMut(i8)>(mut f: F, i: i8) {
    f(i);
}

fn ret_closure<'a>(x: &'a mut XNum, t: bool) -> Box<dyn FnMut(i8) + 'a> {
    if t {
        Box::new(|a| x.incr(a))
    } else {
        Box::new(|b| x.decr(b))
    }
}

#[derive(Debug)]
struct XNum {
    v: i8,
}

impl XNum {
    fn incr(&mut self, i: i8) {
        self.v += i;
    }
    fn decr(&mut self, i: i8) {
        self.v -= i;
    }
}

fn test0() {
    let mut x = XNum { v: 5 };

    let f = ret_closure(&mut x, true);
    foo(f, 2);

    // if true {
    //     foo(|a| x.incr(a), 2);
    // } else {
    //     let f = |b| x.decr(b);
    //     foo(f, 3);
    // }

    println!("{:?}", x);
}

fn main() {
    test0();
}
