#![no_std]
#![feature(alloc_error_handler)]
#![feature(const_mut_refs)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_util::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

#[cfg(test)]
use debug::debug;
#[cfg(test)]
use bootloader::entry_point;

#[cfg(test)]
pub mod test_util;

#[cfg(test)]
entry_point!(test_kernel_main);
fn test_kernel_main(boot_info: &'static bootloader::BootInfo) -> ! {
    test_util::test_kernel_main(boot_info)
}

#[cfg(test)]
#[panic_handler]
fn panic_handler(info: &core::panic::PanicInfo) -> ! {
    test_util::test_panic_handler(info)
}

mod memory;

pub mod kernel_init;
