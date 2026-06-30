#![doc = include_str!("../readme.md")]

/// The path to the built `uasm` executable. for now this will be a &str until 
/// const Path::new gets moved to stable - https://github.com/rust-lang/rust/issues/143773
pub const UASM_PATH: &str = env!("UASM_PATH");

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;

    #[test]
    fn run_uasm() -> Result<(), Box<dyn Error>> {
        let status = std::process::Command::new(UASM_PATH).arg("-h").status()?;
        assert!(
            status.success(),
            "uasm returned with non-zero exit code: {}",
            status.code().unwrap()
        );
        Ok(())
    }
}
