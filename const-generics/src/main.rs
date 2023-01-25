trait Max {
    fn max(&self) -> bool {
        true
    }
}

impl<const N: usize> Max for Cat<N> {
    fn max(&self) -> bool {
        self.id.len() <= 2
    }
}

#[derive(Debug)]
struct Cat<const N: usize> {
    id: [u32; N],
}

#[derive(Debug)]
struct Foo<T>
where
    T: Max,
{
    v: T,
}

fn main() {
    let cs = Cat { id: [8, 9, 10] };

    println!("{:?}", cs.max());

    let fs0 = Foo { v: cs };

    println!("{:?}", fs0);
}
