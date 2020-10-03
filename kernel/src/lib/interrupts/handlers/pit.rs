use x86_64::structures::idt::InterruptStackFrame;
use crate::interrupts::interrupt_index::{PICS, InterruptIndex};
use conquer_once::spin::OnceCell;

pub static PIT_HANDLER: OnceCell<fn(&mut InterruptStackFrame)> = OnceCell::uninit();

pub extern "x86-interrupt" fn pit_handler(
    stack_frame: &mut InterruptStackFrame)
{
    match PIT_HANDLER.try_get() {
        Ok(handler) => {
            crate::debug!("Calling pit handler");
            handler(stack_frame)
        },
        Err(error) => crate::debug!("Error getting pit handler: {:?}", error)
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}
