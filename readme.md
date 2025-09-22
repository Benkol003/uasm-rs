### uasm-rs
A library to build the UASM compiler, for usage in build scripts.

#### Usage
This crate provides one export, which is the path to the built UASM executable.

Example usage:
```rs
use std::result::Result;
use std::error::Error;

use uasm::UASM_PATH;

fn main() -> Result<(),Box<dyn Error>>{
    println!("Hello, world!");
    std::process::Command::new(UASM_PATH).arg("-h").spawn()?;
    Ok(())
}
```
(:
