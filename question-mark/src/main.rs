fn max<'a>(a: &'a str, b: &'a str) -> Option<&'a str> {
    if a.len() > b.len() {
        Some(a)
    } else if a.len() < b.len() {
        Some(b)
    } else {
        None
    }
}

fn test0() -> Option<String> {
    let a = "hello";
    let b = "a";

    let r = max(a, b)?;

    println!("max(a, b) = {:?}", r);

    Some(r.to_string())
}

#[derive(Debug)]
struct Error<'a> {
    e: &'a str,
}

fn foo<'a>(t: bool) -> Result<String, Error<'a>> {
    if t {
        Ok("ok".to_string())
    } else {
        Err(Error { e: "error" })
    }
}

fn test1<'a>() -> Result<String, Error<'a>> {
    let r = foo(true)?;
    println!("foo(true) = {:?}", r);

    let r = foo(false)?;
    println!("foo(false) = {:?}", r);

    Ok(r)
}

fn main() {
    let t0 = test0();
    println!("t0 = {:?}", t0);

    let t1 = test1();
    println!("t1 = {:?}", t1);
}
