mod i686;

#[cfg(target_arch = "x86")]
pub use i686::*;

use core::fmt;

#[derive(Default)]
struct Tty;

impl fmt::Write for Tty {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        print(s);
        Ok(())
    }
}

pub fn idle() -> ! {
    unsafe { asm!("hlt") };
    unreachable!();
}

#[doc(hidden)]
pub(crate) fn _print(args: fmt::Arguments) {
    use fmt::Write;
    Tty::default().write_fmt(args).unwrap();
}
