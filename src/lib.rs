#![doc = include_str!("../readme.md")]

/// The path to the built `uasm` executable.
pub const UASM_PATH: &str = env!("UASM_PATH");

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
