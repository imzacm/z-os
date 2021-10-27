#[cfg(target_arch = "x86")]
mod x86;

#[cfg(target_arch = "x86")]
pub use x86::*;

use core::fmt;
use fmt::Write;

#[doc(hidden)]
pub(crate) fn _print(args: fmt::Arguments) {
    get_tty().write_fmt(args).unwrap();
}
