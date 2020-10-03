use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};
use crate::hlt_loop;
use conquer_once::spin::OnceCell;

pub static PAGE_FAULT_HANDLER: OnceCell<fn(&mut InterruptStackFrame)> = OnceCell::uninit();

pub extern "x86-interrupt" fn page_fault_handler(
    stack_frame: &mut InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    crate::debug!("EXCEPTION: PAGE FAULT");
    crate::debug!("Accessed Address: {:?}", Cr2::read());
    crate::debug!("Error Code: {:?}", error_code);
    crate::debug!("{:#?}", stack_frame);

    match PAGE_FAULT_HANDLER.try_get() {
        Ok(handler) => {
            crate::debug!("Calling page fault handler");
            handler(stack_frame)
        },
        Err(error) => crate::debug!("Error getting page fault handler: {:?}", error)
    }
    hlt_loop();
}
