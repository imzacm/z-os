mod bitmasks {
    pub const OUTPUT_BUFFER_STATUS: u8 = 1 << 0;
    pub const INPUT_BUFFER_STATUS: u8 = 1 << 1;
    pub const TIMEOUT_ERROR: u8 = 1 << 6;
    pub const PARITY_ERROR: u8 = 1 << 7;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Status(u8);

impl Status {
    pub fn new(value: u8) -> Self {
        Self(value)
    }

    pub fn output_buffer_empty(&self) -> bool {
        (self.0 & bitmasks::OUTPUT_BUFFER_STATUS) == 0
    }

    pub fn input_buffer_empty(&self) -> bool {
        (self.0 & bitmasks::INPUT_BUFFER_STATUS) == 0
    }
}
