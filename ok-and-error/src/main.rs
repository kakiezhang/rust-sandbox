use anyhow::{anyhow, Result};

fn main() {
    let rs = bar();
    println!("rs: {:?}", rs);
}

fn bar() -> Result<i32> {
    // let a = 1;
    let a = -1;

    let i = foo(a)
        .and_then(|x| Ok(x))
        .map_err(|e| return anyhow!("error code: {:?}", e.to_string()))?;
    println!("i: {:?}", i);

    Ok(i)
}

fn foo(x: i32) -> Result<i32> {
    if x > 0 {
        Ok(x)
    } else {
        Err(anyhow!("< 0"))
    }
}
