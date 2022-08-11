#[cfg(target_arch = "powerpc64le")]
pub use arch_specific::*;

mod arch_specific {
    use anyhow::Result;

    pub fn check(value: &str) -> Result<()> {
        println!("INFO: check: powerpc64le: value: {:?}", value);

        Ok(())
    }
}
