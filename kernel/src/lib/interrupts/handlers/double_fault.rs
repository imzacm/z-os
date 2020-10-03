use x86_64::structures::idt::InterruptStackFrame;
use conquer_once::spin::OnceCell;
use crate::hlt_loop;

pub static DOUBLE_FAULT_HANDLER: OnceCell<fn(&mut InterruptStackFrame)> = OnceCell::uninit();

pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: &mut InterruptStackFrame, _error_code: u64) -> !
{
    crate::debug!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    match DOUBLE_FAULT_HANDLER.try_get() {
        Ok(handler) => {
            crate::debug!("Calling double fault handler");
            handler(stack_frame)
        },
        Err(error) => crate::debug!("Error getting double fault handler: {:?}", error)
    }
    hlt_loop()
}
