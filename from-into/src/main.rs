fn main() {
    // test_dto();
    // test0()
    test1();
}

fn test_dto() {
    let c = Animal {
        id: 1,
        name: "miao".to_string(),
    };
    println!("c: {:?}, {:p}", c, &c);

    // c1 & c2 are the same ways，both execute AnimalDto::from method
    let mut c1: AnimalDto = c.clone().into();
    println!(
        "c1: {:?}, {:p} | name: {:?}, {:p}",
        c1,
        &c1,
        c1.name,
        c1.name.as_ptr()
    );

    // c.clone() make c and its field name all to clone
    let c2 = AnimalDto::from(c.clone());
    println!(
        "c2: {:?}, {:p} | name: {:?}, {:p}",
        c2,
        &c2,
        c2.name,
        c2.name.as_ptr(),
    );

    c1.name = String::from("miao2");

    println!(
        "after change c1: {:?}, {:p} | name: {:?}, {:p}",
        c1,
        &c1,
        c1.name,
        c1.name.as_ptr()
    );

    println!(
        "after change c2: {:?}, {:p} | name: {:?}, {:p}",
        c2,
        &c2,
        c2.name,
        c2.name.as_ptr(),
    );
}

fn test0() {
    let c = Animal {
        id: 1,
        name: "miao".to_string(),
    };
    println!("c: {:?}, {:p}", c, &c);

    // c3 & c4 are the same ways
    let c3: Animal = c.clone().into();
    println!("c3: {:?}, {:p}", c3, &c3);

    let c4 = Animal::from(c);
    println!("c4: {:?}, {:p}", c4, &c4);
}

fn test1() {
    let c0 = &Cat {
        id: 2,
        name: "miumiu".into(),
    };

    let c1 = Cat::new();
    println!("{:?}", &c1);
    t1(c1);
}

#[derive(Debug, Clone)]
struct Animal {
    id: u8,
    name: String,
}

#[derive(Debug)]
struct AnimalDto {
    id: u32,
    name: String,
}

impl From<Animal> for AnimalDto {
    fn from(a: Animal) -> Self {
        println!("AnimalDto from inital.");
        AnimalDto {
            id: a.id as u32,
            name: a.name.into(),
        }
    }
}

#[derive(Debug)]
struct Cat<'a> {
    id: u32,
    name: &'a str,
}

impl<'a> Cat<'a> {
    fn new() -> Self {
        Cat {
            id: 8,
            name: "hello",
        }
    }
}

fn t1(c: Cat) {
    println!("{:?}", c);
}
