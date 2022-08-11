#[cfg(target_arch = "s390x")]
pub use arch_specific::*;

mod arch_specific {
    use anyhow::Result;

    pub fn check(value: &str) -> Result<()> {
        println!("INFO: check: s390x: value: {:?}", value);

        Ok(())
    }
}
