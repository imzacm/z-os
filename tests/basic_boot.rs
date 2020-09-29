#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(z_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};
use z_os::{println, hlt_loop};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    z_os::test_panic_handler(info)
}

entry_point!(test_kernel_main);

fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    test_main();
    hlt_loop();
}


#[test_case]
fn test_println() {
    println!("test_println output");
}
