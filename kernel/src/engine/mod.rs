mod exceptions;
mod interrupts;

pub use exceptions::{Exception, handle_exception};
pub use interrupts::{Interrupt, handle_interrupt};

pub fn poll() {}
