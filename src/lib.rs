//!
//!This crate provides one export, which is the path to the built UASM executable.
//!
//!Example usage:
//!```rust
//!use std::result::Result;
//!use std::error::Error;
//!
//!use uasm::UASM_PATH;
//!
//!fn main() -> Result<(),Box<dyn Error>>{
//!    println!("Hello, world!");
//!    std::process::Command::new(UASM_PATH).arg("-h").spawn()?;
//!    Ok(())
//!}
//!```

/// The path to the built ```uasm``` executable.
pub const UASM_PATH: &'static str = env!("UASM_PATH");

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    #[test]
    fn run_uasm() -> Result<(),Box<dyn Error>> {
        let status = std::process::Command::new(UASM_PATH).arg("-h").status()?;
        assert!(status.success(),"uasm returned with non-zero exit code: {}",status.code().unwrap());
        Ok(())
    }
}
