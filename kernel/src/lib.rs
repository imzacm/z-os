#![feature(asm)]
#![feature(llvm_asm)]
#![feature(exclusive_range_pattern)]
#![deny(clippy::all)]
#![no_std]

use core::panic::PanicInfo;
use crate::arch::print;

mod hardware;
mod syscalls;
mod arch;

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ($crate::arch::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! kprintln {
    () => ($crate::kprint!("\n"));
    ($($arg:tt)*) => ($crate::kprint!("{}\n", format_args!($($arg)*)));
}

#[no_mangle]
extern "C" fn kernel_main() -> ! {
    arch::init();
    kprintln!("z-os v{}", env!("CARGO_PKG_VERSION"));
    arch::idle();
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    kprintln!("Kernel panic: {}", info);
    arch::idle();
}
