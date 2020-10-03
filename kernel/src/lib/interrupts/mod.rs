pub mod interrupt_index;
pub mod handlers;
pub mod idt;

pub fn init_interrupts() {
    idt::init_idt();
    unsafe { interrupt_index::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable();
}
