use x86_64::structures::idt::InterruptStackFrame;
use crate::interrupts::interrupt_index::{PICS, InterruptIndex};
use conquer_once::spin::OnceCell;

pub static PS2_KEYBOARD_HANDLER: OnceCell<fn(&mut InterruptStackFrame)> = OnceCell::uninit();

pub extern "x86-interrupt" fn ps2_keyboard_handler(
    stack_frame: &mut InterruptStackFrame)
{
    match PS2_KEYBOARD_HANDLER.try_get() {
        Ok(handler) => {
            handler(stack_frame)
        },
        Err(error) => crate::debug!("Error getting double fault handler: {:?}", error)
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}
