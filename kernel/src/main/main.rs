#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(z_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use z_os::println;
use z_os::task::{Task, executor::Executor, keyboard};

mod ui;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    z_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    z_os::test_panic_handler(info)
}

async fn async_number() -> u32 {
    42
}

async fn example_task() {
    let number = async_number().await;
    println!("async number: {}", number);
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use z_os::memory;
    use z_os::allocator;
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");

    z_os::init(Some(boot_info));

    #[cfg(test)]
        test_main();

    ui::enter_ui()
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
