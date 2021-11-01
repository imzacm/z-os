mod bitmasks {
    pub const FIRST_PORT_INTERRUPT: u8 = 1 << 0;
    pub const SECOND_PORT_INTERRUPT: u8 = 1 << 1;
    pub const FIRST_PORT_CLOCK: u8 = 1 << 4;
    pub const SECOND_PORT_CLOCK: u8 = 1 << 5;
    pub const FIRST_PORT_TRANSLATION: u8 = 1 << 6;
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ControllerConfig(u8);

impl ControllerConfig {
    pub fn new(value: u8) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u8 {
        self.0
    }

    pub fn first_port_interrupt(&self) -> bool {
        (self.0 & bitmasks::FIRST_PORT_INTERRUPT) != 0
    }

    pub fn set_first_port_interrupt(self, enabled: bool) -> Self {
        let value = if enabled {
            self.0 | bitmasks::FIRST_PORT_INTERRUPT
        } else {
            self.0 & !(bitmasks::FIRST_PORT_INTERRUPT)
        };
        Self(value)
    }

    pub fn second_port_interrupt(&self) -> bool {
        (self.0 & bitmasks::SECOND_PORT_INTERRUPT) != 0
    }

    pub fn set_second_port_interrupt(self, enabled: bool) -> Self {
        let value = if enabled {
            self.0 | bitmasks::SECOND_PORT_INTERRUPT
        } else {
            self.0 & !(bitmasks::SECOND_PORT_INTERRUPT)
        };
        Self(value)
    }

    pub fn first_port_clock(&self) -> bool {
        (self.0 & bitmasks::FIRST_PORT_CLOCK) != 0
    }

    pub fn set_first_port_clock(self, enabled: bool) -> Self {
        let value = if enabled {
            self.0 | bitmasks::FIRST_PORT_CLOCK
        } else {
            self.0 & !(bitmasks::FIRST_PORT_CLOCK)
        };
        Self(value)
    }

    pub fn second_port_clock(&self) -> bool {
        (self.0 & bitmasks::SECOND_PORT_CLOCK) != 0
    }

    pub fn set_second_port_clock(self, enabled: bool) -> Self {
        let value = if enabled {
            self.0 | bitmasks::SECOND_PORT_CLOCK
        } else {
            self.0 & !(bitmasks::SECOND_PORT_CLOCK)
        };
        Self(value)
    }

    pub fn first_port_translation(&self) -> bool {
        (self.0 & bitmasks::FIRST_PORT_TRANSLATION) != 0
    }

    pub fn set_first_port_translation(self, enabled: bool) -> Self {
        let value = if enabled {
            self.0 | bitmasks::FIRST_PORT_TRANSLATION
        } else {
            self.0 & !(bitmasks::FIRST_PORT_TRANSLATION)
        };
        Self(value)
    }
}
