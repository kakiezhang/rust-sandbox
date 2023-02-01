#[derive(Debug)]
struct Animal {
    id: u32,
    age: u8,
}

impl AsRef<u8> for Animal {
    fn as_ref(&self) -> &u8 {
        &self.age
    }
}

impl AsMut<u8> for Animal {
    fn as_mut(&mut self) -> &mut u8 {
        &mut self.age
    }
}

fn main() {
    {
        let s = "hello";
        println!("s: {:?}, {:?}", s, s.as_ptr());

        let s1: &str = s.as_ref();
        println!("s1: {:?}, {:?}", s1, s1.as_ptr());

        // turbofish
        let s2 = AsRef::<str>::as_ref(s);
        println!("s2: {:?}, {:?}", s2, s2.as_ptr());

        let s3: &[u8] = s.as_ref();
        println!("s3: {:?}, {:?}", s3, s3.as_ptr());

        // turbofish
        let s4 = AsRef::<[u8]>::as_ref(s);
        println!("s4: {:?}, {:?}", s4, s4.as_ptr());

        let s5 = &s;
        println!("s5: {:?}, {:?}", s5, s5.as_ptr());
    }

    {
        let i = Box::new(5i8);
        println!("i: {:?}, {:p}", i, i);

        let i1: &i8 = i.as_ref();
        println!("i1: {:?}, {:p}", i1, i1);

        let i2: &i8 = &i;
        println!("i2: {:?}, {:p}", i2, i2);
    }

    {
        let mut a = Animal {
            id: 100001,
            age: 10,
        };
        println!("a: {:?}, {:p}", a, &a);

        let a1 = a.as_ref();
        println!("a1: {:?}, {:p}", a1, a1);

        let a2 = &a.age;
        println!("a2: {:?}, {:p}", a2, a2);

        // let a3 = &mut a;
        // a3.age = 12;
        // println!("a3: {:?}, {:p}", a3, a3);

        *a.as_mut() = 20;
        println!("a: {:?}, {:p}", a, &a);

        let mut a4 = *a.as_mut();
        a4 = 2;
        println!("a4: {:?}, {:p}", a4, &a4);
    }
}
