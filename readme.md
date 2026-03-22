### uasm-rs
A library to build the UASM compiler, for usage in build scripts.

#### Usage
This crate provides the path to the built UASM executable.

Example usage:

```rust
use std::result::Result;
use std::error::Error;

use uasm::UASM_PATH;

fn main() -> Result<(),Box<dyn Error>>{
    std::process::Command::new(UASM_PATH).arg("-h").spawn()?;
    Ok(())
}
```
or from a `build.rs` script:
```rust,no_run
use std::result::Result;
use std::error::Error;

use uasm::UASM_PATH;

fn main() -> Result<(),Box<dyn Error>>{
    let uasm = std::env::var("DEP_UASM_PATH").unwrap();
    std::process::Command::new(uasm).arg("-h").spawn()?;
    Ok(())
}
```
