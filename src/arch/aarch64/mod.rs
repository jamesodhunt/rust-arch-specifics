#[cfg(target_arch = "aarch64")]
pub use arch_specific::*;

mod arch_specific {
    use anyhow::Result;

    pub fn check(value: &str) -> Result<()> {
        println!("INFO: check: aarch64: value: {:?}", value);

        Ok(())
    }
}
