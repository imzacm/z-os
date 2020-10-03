use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;
use super::interrupt_index::InterruptIndex;
use super::handlers::breakpoints::breakpoint_handler;
use super::handlers::double_fault::double_fault_handler;
use super::handlers::page_fault::page_fault_handler;
use super::handlers::pit::pit_handler;
use super::handlers::ps2::ps2_keyboard_handler;
use crate::gdt::tss::DOUBLE_FAULT_IST_INDEX;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX);
        }
        idt.page_fault.set_handler_fn(page_fault_handler);

        idt[InterruptIndex::Timer.as_usize()]
            .set_handler_fn(pit_handler);
        idt[InterruptIndex::Keyboard.as_usize()]
            .set_handler_fn(ps2_keyboard_handler);

        idt
    };
}

pub fn init_idt() {
    IDT.load();
}
