/// Only covers recoverable exceptions, all others should `panic!()`
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Exception {
    Overflow,
    BoundRangeExceeded,
    InvalidInstruction,
    DeviceNotAvailable,
    /// Covers InvalidTss, StackSegmentFault, GeneralProtectionFault, and PageFault
    MemoryFault,
    Virtualization,
    Security,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Interrupt {
    Timer,
}

// TODO: Add exception detail
pub fn handle_exception(exception: Exception) {
    crate::kprintln!("Exception: {:?}", exception);
}

// TODO: Add interrupt detail
pub fn handle_interrupt(interrupt: Interrupt) {
    crate::kprintln!("Interrupt: {:?}", interrupt);
}
