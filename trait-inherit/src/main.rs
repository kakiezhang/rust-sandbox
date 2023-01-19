trait Bark {
    fn bark(&self);
}

trait BarkExt: Bark {
    fn bark_twice(&self) {
        self.bark();
        self.bark();
    }
}

#[derive(Debug)]
struct Dog<'a> {
    name: &'a str,
}

impl<T: ?Sized> BarkExt for T
where
    T: Bark,
{
    // add code here
}

// impl<'a> BarkExt for Dog<'a>
// where
//     Dog<'a>: Bark,
// {
//     // add code here
// }

impl<'a> Bark for Dog<'a> {
    fn bark(&self) {
        println!("{} will wang", self.name);
    }
}

fn main() {
    let d = Dog {
        name: "john".into(),
    };
    d.bark_twice();
    // d.bark();
}
