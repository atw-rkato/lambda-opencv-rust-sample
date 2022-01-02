use anyhow::{bail, Context, Result};

pub fn run() -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::run;

    #[test]
    fn run_test() {
        let _ = env_logger::builder().is_test(true).try_init();
        assert!(run().is_ok());
    }
}
