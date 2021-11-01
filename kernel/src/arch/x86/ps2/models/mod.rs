mod keyboard;
mod mouse;

pub use keyboard::KeyboardModel;
pub use mouse::MouseModel;

use crate::engine::HumanInput;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Ps2ModelError {
    ScancodeQueueFull,
}

pub trait Ps2Model {
    fn scancode_queue_capacity(&self) -> usize;

    fn scancode_queue_len(&self) -> usize;

    fn push_scancode(&mut self, scancode: u8) -> Result<(), Ps2ModelError>;

    fn pop_input(&mut self) -> Result<Option<HumanInput>, Ps2ModelError>;

    fn clear(&mut self);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Ps2DeviceType {
    AtKeyboard,
    StandardMouse,
    ScrollMouse,
    FiveBtnMouse,
    Mf2Keyboard,
}

#[derive(Debug)]
pub enum Ps2DeviceModel<const SCANCODE_CAP: usize> {
    Keyboard(KeyboardModel<SCANCODE_CAP>),
    Mouse(MouseModel<SCANCODE_CAP>),
}

impl<const SCANCODE_CAP: usize> From<Ps2DeviceType> for Ps2DeviceModel<SCANCODE_CAP> {
    fn from(device_type: Ps2DeviceType) -> Self {
        match device_type {
            Ps2DeviceType::AtKeyboard | Ps2DeviceType::Mf2Keyboard => {
                Self::Keyboard(KeyboardModel::new())
            }
            _ => Self::Mouse(MouseModel::new())
        }
    }
}

impl<const SCANCODE_CAP: usize> Ps2Model for Ps2DeviceModel<SCANCODE_CAP> {
    fn scancode_queue_capacity(&self) -> usize {
        match self {
            Self::Keyboard(model) => model.scancode_queue_capacity(),
            Self::Mouse(model) => model.scancode_queue_capacity(),
        }
    }

    fn scancode_queue_len(&self) -> usize {
        match self {
            Self::Keyboard(model) => model.scancode_queue_len(),
            Self::Mouse(model) => model.scancode_queue_len(),
        }
    }

    fn push_scancode(&mut self, scancode: u8) -> Result<(), Ps2ModelError> {
        match self {
            Self::Keyboard(model) => model.push_scancode(scancode),
            Self::Mouse(model) => model.push_scancode(scancode),
        }
    }

    fn pop_input(&mut self) -> Result<Option<HumanInput>, Ps2ModelError> {
        match self {
            Self::Keyboard(model) => model.pop_input(),
            Self::Mouse(model) => model.pop_input(),
        }
    }

    fn clear(&mut self) {
        match self {
            Self::Keyboard(model) => model.clear(),
            Self::Mouse(model) => model.clear(),
        }
    }
}
