use x86_64::structures::idt::InterruptStackFrame;
use conquer_once::spin::OnceCell;
use alloc::boxed::Box;

pub static BREAKPOINT_HANDLER: OnceCell<fn(&mut InterruptStackFrame)> = OnceCell::uninit();

pub extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: &mut InterruptStackFrame)
{
    crate::debug!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
    match BREAKPOINT_HANDLER.try_get() {
        Ok(handler) => {
            crate::debug!("Calling breakpoint handler");
            handler(stack_frame)
        },
        Err(error) => crate::debug!("Error getting breakpoint handler: {:?}", error)
    }
}

#[test_case]
fn test_breakpoint_exception() {
    x86_64::instructions::interrupts::int3();
}
