use core::result::Result;
use pc_keyboard::{KeyCode, KeyState, Modifiers};
use core::fmt::Debug;
use crate::drivers::{Box, DriverError, Driver};

#[derive(Debug, Copy, Clone)]
pub struct KeyPress {
    pub scan_code: u8,
    pub modifiers: Modifiers,
    pub key_state: KeyState,
    pub key_code: KeyCode,
    pub unicode: Option<char>,
}

pub trait KeyboardDriver: Driver {
    fn read_key(&self) -> Result<Option<KeyPress>, DriverError>;

    fn simulate_key(&self) -> Result<(), DriverError>;
}
