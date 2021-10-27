mod interrupts;

pub use interrupts::{Exception, Interrupt, handle_exception, handle_interrupt};

pub fn poll() {}
