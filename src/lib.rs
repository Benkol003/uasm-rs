
pub const UASM_PATH: &'static str = env!("UASM_PATH");

fn main() {
    println!("uasm @ {}",UASM_PATH);
    let status = std::process::Command::new(UASM_PATH).spawn();
}

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
