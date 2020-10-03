use core::panic::PanicInfo;
use bootloader::BootInfo;
use x86_64::VirtAddr;
use crate::kernel_init::KernelInit;

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
    where
        T: Fn(),
{
    fn run(&self) {
        crate::debug!("{}...\t", core::any::type_name::<T>());
        self();
        crate::debug!("[ok]");
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    crate::debug!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    crate::debug!("[failed]\n");
    crate::debug!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

pub fn test_kernel_main(boot_info: &'static BootInfo) -> ! {
    KernelInit::default(Some(boot_info)).apply().unwrap();
    crate::test_main();
    hlt_loop();
}

pub fn panic_handler(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[alloc_error_handler]
pub fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
