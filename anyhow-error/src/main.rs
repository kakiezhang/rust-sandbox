use anyhow::{anyhow, bail, Result};
use thiserror::Error;

fn main() {
    // {
    //     let a0 = foo_0(1);
    //     println!("a0: {:?}", a0);

    //     let a1 = foo_0(2);
    //     println!("a1: {:?}", a1);
    // }

    // {
    //     let b0 = foo_1(10);
    //     println!("b0: {:?}", b0);

    //     let b1 = foo_1(128);
    //     println!("b1: {:?}", b1);
    // }

    // {
    //     let c0 = foo_2(128);
    //     println!("c0: {:?}", c0);

    //     let c1 = foo_2(10);
    //     println!("c1: {:?}", c1);
    // }

    let e = SysError::Interrupt(20);
    match e {
        SysError::Interrupt(0) => expr,
    }
}

// fn foo_0(i: u8) -> Result<&'static str> {
//     if i == 1 {
//         Ok("hello")
//     } else {
//         Err(anyhow!("Hi"))
//     }
// }

#[derive(Error, Debug)]
enum SysError {
    #[error("idx: {0}, code: {1}")]
    Interrupt{ idx: u16, code: 1234},
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
        let e = IOError::InvalidDoubleItem(127);
        match e {
            IOError::InvalidDoubleItem(code) => {
                println!("code: {:?}", code)
            }
        }
        Err(e)
    } else {
        Ok(i + i)
    }
}

// fn foo_2(i: u8) -> Result<()> {
//     if i > 127 {
//         // for early return
//         bail!(IOError::InvalidDoubleItem(128));
//     }
//     Ok(())
// }
