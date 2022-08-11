use anyhow::{anyhow, Result};

#[cfg(target_arch = "aarch64")]
pub mod aarch64;

#[cfg(target_arch = "powerpc64le")]
pub mod powerpc64le;

#[cfg(target_arch = "s390x")]
pub mod s390x;

#[cfg(target_arch = "x86_64")]
pub mod x86_64;

pub fn check(value: &str) -> Result<()> {
    println!("INFO: arch::check: value: {:?}", value);

    #[cfg(target_arch = "aarch64")]
    let result = aarch64::check(value);

    #[cfg(target_arch = "powerpc64le")]
    let result = powerpc64le::check(value);

    #[cfg(target_arch = "s390x")]
    let result = s390x::check(value);

    #[cfg(target_arch = "x86_64")]
    let result = x86_64::check(value);

    #[cfg(not(any(
        target_arch = "aarch64",
        target_arch = "powerpc64le",
        target_arch = "s390x",
        target_arch = "x86_64"
    )))]
    panic!("unknown architecture");

    result
}
