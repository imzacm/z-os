#![deny(clippy::all)]
#![no_std]

use core::panic::PanicInfo;

mod hardware;
mod syscalls;
mod arch;

#[no_mangle]
extern "C" fn kernel_main() -> ! {
    arch::init();
    #[allow(clippy::empty_loop)]
    loop {}
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}
