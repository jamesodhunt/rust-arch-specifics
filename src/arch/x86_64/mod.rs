#[cfg(target_arch = "x86_64")]
pub use arch_specific::*;

mod arch_specific {
    use anyhow::Result;

    pub fn check(value: &str) -> Result<()> {
        println!("INFO: check: x86_64: value: {:?}", value);

        Ok(())
    }
}
