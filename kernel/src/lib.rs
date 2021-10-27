#![feature(asm)]
#![feature(llvm_asm)]
#![feature(core_intrinsics)]
#![feature(exclusive_range_pattern)]
#![deny(clippy::all)]
#![no_std]

use core::panic::PanicInfo;

mod arch;
mod engine;

#[macro_export]
macro_rules! kprint {
    ($($arg:tt)*) => ($crate::arch::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! kprintln {
    () => ($crate::kprint!("\n"));
    ($($arg:tt)*) => ($crate::kprint!("{}\n", format_args!($($arg)*)));
}

pub(crate) fn idle() -> ! {
    loop {
        engine::poll();
        arch::idle();
    }
}

#[no_mangle]
extern "C" fn kernel_main() -> ! {
    kprintln!("z-os v{}", env!("CARGO_PKG_VERSION"));
    idle();
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    kprintln!("Kernel panic: {}", info);
    // Manually enter a halt loop because the system must not continue running
    arch::disable_interrupts();
    loop {
        arch::idle();
    }
}
