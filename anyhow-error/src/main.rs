use anyhow::{anyhow, Result};
use thiserror::Error;

fn main() {
    {
        let a0 = foo_0(1);
        println!("a0: {:?}", a0);

        let a1 = foo_0(2);
        println!("a1: {:?}", a1);
    }

    {
        let b0 = foo_1(10);
        println!("b0: {:?}", b0);

        let b1 = foo_1(128);
        println!("b1: {:?}", b1);
    }
}

fn foo_0(i: u8) -> Result<&'static str> {
    if i == 1 {
        Ok("hello")
    } else {
        Err(anyhow!("Hi"))
    }
}

type IOResult<T> = core::result::Result<T, IOError>;

#[derive(Error, Debug)]
enum IOError {
    #[error("invalid double item {0}")]
    InvalidDoubleItem(u16),
}

fn foo_1(i: u8) -> Result<u8> {
    let d = double_1(i)?;
    Ok(d)
}

fn double_1(i: u8) -> IOResult<u8> {
    if i > 127 {
        // Err(anyhow!("invalid double item {}", 127))
        Err(IOError::InvalidDoubleItem(127))
    } else {
        Ok(i + i)
    }
}
