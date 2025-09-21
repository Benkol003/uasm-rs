### uasm-rs
A library for Cargo build scripts for providing the UASM assembler.

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
