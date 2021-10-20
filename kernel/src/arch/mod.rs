mod i686;

#[cfg(target_arch = "x86")]
pub use i686::*;
