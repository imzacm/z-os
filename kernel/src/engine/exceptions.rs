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

pub fn handle_exception(exception: Exception, instruction_pointer: u32) {
    crate::kprintln!("Exception {:?} at instruction {:#08x}", exception, instruction_pointer);
}
