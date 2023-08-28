fn main() {
    println!("foo(Ok(1)) = {:?}", foo(Ok(1)));
    println!("foo(Err(1)) = {:?}", foo(Err(1)));
}

fn foo(a: Result<i32, i32>) -> Result<i32, String> {
    let a = match a {
        Ok(o) => o,
        Err(e) => {
            return Err(format!("errcode: {e}", e = e));
        }
    };

    Ok(a * 10)
}
