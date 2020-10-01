#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(z_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point};
use z_os::{println, ui};
use z_os::drivers::display::vga::{VgaDisplayDriver, Color, VGA_WIDTH, VGA_HEIGHT};
use z_os::drivers::keyboard::ps2::Ps2Keyboard;
use z_os::drivers::text_cursor::TextModeCursor;
use z_os::task::executor::Executor;
use z_os::task::Task;

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

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use z_os::memory;
    use z_os::allocator;
    use x86_64::VirtAddr;

    z_os::init();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    allocator::init_heap(&mut mapper, &mut frame_allocator)
        .expect("heap initialization failed");

    #[cfg(test)]
        test_main();

    let mut text_cursor = TextModeCursor::new(VGA_WIDTH, VGA_HEIGHT);
    let mut vga = VgaDisplayDriver::new(Color::White, Color::Black, &mut text_cursor);
    let mut ui = ui::user_interface::UserInterface::new(&mut vga).unwrap();

    let mut executor = Executor::new();
    executor.spawn(Task::new(ui.run()));
    executor.run()
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
