use super::{Ps2Model, Ps2ModelError};
use crate::engine::{HumanInput, Key, KeyLocation, KeyModifiers, KeySection, KeyState};
use heapless::spsc::Queue;

/*
Sections index map:

Escape:
    0: Escape

Function:
    0..11: F1..F12

Special:
    0: Print screen
    1: Num lock
    2: Scroll lock
    3: Pause/Break
    4: Insert
    5: Delete
    6: Home
    7: End
    8: Page up
    9: Page down

Media:
    TODO

Cursor:
    0: Up
    1: Down
    2: Left
    3: Right

Keypad:
    0: /
    1: *
    2: -
    3: +
    4..14: 0..9
    15: .
    16: Delete
    17: Enter

Left modifiers:
    0: Caps lock
    1: Shift
    2: Control
    3: Super
    4: Alt

Right modifiers:
    0: Shift
    1: Alt
    2: Control

Center:
    0: `
    1..11: 0..9
    12: -
    3: Tab
    4: Backspace
    5: Enter
*/

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Scancode {
    Single(u8),
    Extended(u8, u8),
}

impl Scancode {
    const fn is_single(self) -> bool {
        matches!(self, Self::Single(_))
    }

    const fn is_extended(self) -> bool {
        matches!(self, Self::Extended(_, _))
    }

    const fn first_scancode(self) -> u8 {
        match self {
            Self::Single(scancode) => scancode,
            Self::Extended(scancode, _) => scancode,
        }
    }

    const fn second_scancode(self) -> Option<u8> {
        match self {
            Self::Single(_) => None,
            Self::Extended(_, scancode) => Some(scancode),
        }
    }

    const fn pressed(self) -> bool {
        (self.first_scancode() & 0x80) == 0
    }

    const fn is_set_1_extended_prefix(self) -> bool {
        self.first_scancode() == 0xE0
    }

    const fn set_1_us_location(self) -> Option<KeyLocation> {
        let sequence: (u8, Option<u8>) = (
            if self.pressed() { self.first_scancode() } else { self.first_scancode() & !(0x80) },
            self.second_scancode()
        );
        match sequence {
            // Escape
            (0x01, None) => Some(KeyLocation::new(KeySection::Escape, 0, Key::Escape)),
            // 1
            (0x02, None) => Some(KeyLocation::new(KeySection::Center, 1, Key::Num1)),
            // 2
            (0x03, None) => Some(KeyLocation::new(KeySection::Center, 2, Key::Num2)),
            // 3
            (0x04, None) => Some(KeyLocation::new(KeySection::Center, 3, Key::Num3)),
            // 4
            (0x05, None) => Some(KeyLocation::new(KeySection::Center, 4, Key::Num4)),
            // 5
            (0x06, None) => Some(KeyLocation::new(KeySection::Center, 5, Key::Num5)),
            // 6
            (0x07, None) => Some(KeyLocation::new(KeySection::Center, 6, Key::Num6)),
            // 7
            (0x08, None) => Some(KeyLocation::new(KeySection::Center, 7, Key::Num7)),
            // 8
            (0x09, None) => Some(KeyLocation::new(KeySection::Center, 8, Key::Num8)),
            // 9
            (0x0A, None) => Some(KeyLocation::new(KeySection::Center, 9, Key::Num9)),
            // 0
            (0x0B, None) => Some(KeyLocation::new(KeySection::Center, 10, Key::Num0)),
            // -
            (0x0C, None) => Some(KeyLocation::new(KeySection::Center, 11, Key::Hyphen)),
            // =
            (0x0D, None) => Some(KeyLocation::new(KeySection::Center, 12, Key::Equals)),
            // Backspace
            (0x0E, None) => Some(KeyLocation::new(KeySection::Center, 13, Key::Backspace)),
            // Tab
            (0x0F, None) => Some(KeyLocation::new(KeySection::Center, 14, Key::Tab)),
            // Q
            (0x10, None) => Some(KeyLocation::new(KeySection::Center, 15, Key::Q)),
            // W
            (0x11, None) => Some(KeyLocation::new(KeySection::Center, 16, Key::W)),
            // E
            (0x12, None) => Some(KeyLocation::new(KeySection::Center, 17, Key::E)),
            // R
            (0x13, None) => Some(KeyLocation::new(KeySection::Center, 18, Key::R)),
            // T
            (0x14, None) => Some(KeyLocation::new(KeySection::Center, 19, Key::T)),
            // Y
            (0x15, None) => Some(KeyLocation::new(KeySection::Center, 20, Key::Y)),
            // U
            (0x16, None) => Some(KeyLocation::new(KeySection::Center, 21, Key::U)),
            // I
            (0x17, None) => Some(KeyLocation::new(KeySection::Center, 22, Key::I)),
            // O
            (0x18, None) => Some(KeyLocation::new(KeySection::Center, 23, Key::O)),
            // P
            (0x19, None) => Some(KeyLocation::new(KeySection::Center, 24, Key::P)),
            // [ or {
            (0x1A, None) => Some(KeyLocation::new(KeySection::Center, 25, Key::OpenCurly)),
            // ] or }
            (0x1B, None) => Some(KeyLocation::new(KeySection::Center, 26, Key::CloseCurly)),
            // Enter
            (0x1C, None) => Some(KeyLocation::new(KeySection::Center, 27, Key::Enter)),
            // Left control
            (0x1D, None) => Some(KeyLocation::new(KeySection::LeftModifiers, 2, Key::Control)),
            // A
            (0x1E, None) => Some(KeyLocation::new(KeySection::Center, 28, Key::A)),
            // S
            (0x1F, None) => Some(KeyLocation::new(KeySection::Center, 29, Key::S)),
            // D
            (0x20, None) => Some(KeyLocation::new(KeySection::Center, 30, Key::D)),
            // F
            (0x21, None) => Some(KeyLocation::new(KeySection::Center, 31, Key::F)),
            // G
            (0x22, None) => Some(KeyLocation::new(KeySection::Center, 32, Key::G)),
            // H
            (0x23, None) => Some(KeyLocation::new(KeySection::Center, 33, Key::H)),
            // J
            (0x24, None) => Some(KeyLocation::new(KeySection::Center, 34, Key::J)),
            // K
            (0x25, None) => Some(KeyLocation::new(KeySection::Center, 35, Key::K)),
            // L
            (0x26, None) => Some(KeyLocation::new(KeySection::Center, 36, Key::L)),
            // ; or :
            (0x27, None) => Some(KeyLocation::new(KeySection::Center, 37, Key::SemiColon)),
            // '
            (0x28, None) => Some(KeyLocation::new(KeySection::Center, 38, Key::SingleQuote)),
            // `
            (0x29, None) => Some(KeyLocation::new(KeySection::Center, 39, Key::BackTick)),
            // Left shift
            (0x2A, None) => Some(KeyLocation::new(KeySection::LeftModifiers, 1, Key::Shift)),
            // \
            (0x2B, None) => Some(KeyLocation::new(KeySection::Center, 40, Key::BackSlash)),
            // Z
            (0x2C, None) => Some(KeyLocation::new(KeySection::Center, 41, Key::Z)),
            // X
            (0x2D, None) => Some(KeyLocation::new(KeySection::Center, 42, Key::X)),
            // C
            (0x2E, None) => Some(KeyLocation::new(KeySection::Center, 43, Key::C)),
            // V
            (0x2F, None) => Some(KeyLocation::new(KeySection::Center, 44, Key::V)),
            // B
            (0x30, None) => Some(KeyLocation::new(KeySection::Center, 45, Key::B)),
            // N
            (0x31, None) => Some(KeyLocation::new(KeySection::Center, 46, Key::N)),
            // M
            (0x32, None) => Some(KeyLocation::new(KeySection::Center, 47, Key::M)),
            // , or <
            (0x33, None) => Some(KeyLocation::new(KeySection::Center, 48, Key::Comma)),
            // . or >
            (0x34, None) => Some(KeyLocation::new(KeySection::Center, 49, Key::FullStop)),
            // / or ?
            (0x35, None) => Some(KeyLocation::new(KeySection::Center, 50, Key::ForwardSlash)),
            // Right shift
            (0x36, None) => Some(KeyLocation::new(KeySection::RightModifiers, 0, Key::Shift)),
            // Left alt
            (0x38, None) => Some(KeyLocation::new(KeySection::LeftModifiers, 4, Key::Alt)),
            // Space
            (0x39, None) => Some(KeyLocation::new(KeySection::Center, 51, Key::Space)),
            // Caps lock
            (0x3A, None) => Some(KeyLocation::new(KeySection::LeftModifiers, 0, Key::CapsLock)),
            // F1
            (0x3B, None) => Some(KeyLocation::new(KeySection::Function, 0, Key::F1)),
            // F2
            (0x3C, None) => Some(KeyLocation::new(KeySection::Function, 1, Key::F2)),
            // F3
            (0x3D, None) => Some(KeyLocation::new(KeySection::Function, 2, Key::F3)),
            // F4
            (0x3E, None) => Some(KeyLocation::new(KeySection::Function, 3, Key::F4)),
            // F5
            (0x3F, None) => Some(KeyLocation::new(KeySection::Function, 4, Key::F5)),
            // F6
            (0x40, None) => Some(KeyLocation::new(KeySection::Function, 5, Key::F6)),
            // F7
            (0x41, None) => Some(KeyLocation::new(KeySection::Function, 6, Key::F7)),
            // F8
            (0x42, None) => Some(KeyLocation::new(KeySection::Function, 7, Key::F8)),
            // F9
            (0x43, None) => Some(KeyLocation::new(KeySection::Function, 8, Key::F9)),
            // F10
            (0x44, None) => Some(KeyLocation::new(KeySection::Function, 9, Key::F10)),
            // F11
            (0x57, None) => Some(KeyLocation::new(KeySection::Function, 10, Key::F11)),
            // F12
            (0x58, None) => Some(KeyLocation::new(KeySection::Function, 11, Key::F12)),
            // Num lock
            (0x45, None) => Some(KeyLocation::new(KeySection::Special, 1, Key::NumLock)),
            // Scroll lock
            (0x46, None) => Some(KeyLocation::new(KeySection::Special, 2, Key::ScrollLock)),
            // Right alt
            (0xE0, Some(0x38)) => Some(KeyLocation::new(KeySection::RightModifiers, 1, Key::Alt)),
            // Home
            (0xE0, Some(0x47)) => Some(KeyLocation::new(KeySection::Special, 6, Key::Home)),
            // End
            (0xE0, Some(0x4F)) => Some(KeyLocation::new(KeySection::Special, 7, Key::End)),
            // Insert
            (0xE0, Some(0x52)) => Some(KeyLocation::new(KeySection::Special, 4, Key::Insert)),
            // Delete
            (0xE0, Some(0x53)) => Some(KeyLocation::new(KeySection::Special, 5, Key::Delete)),
            // Page up
            (0xE0, Some(0x49)) => Some(KeyLocation::new(KeySection::Special, 8, Key::PageUp)),
            // Page down
            (0xE0, Some(0x51)) => Some(KeyLocation::new(KeySection::Special, 9, Key::PageDown)),
            // Cursor up
            (0xE0, Some(0x48)) => Some(KeyLocation::new(KeySection::Cursor, 0, Key::CursorUp)),
            // Cursor down
            (0xE0, Some(0x50)) => Some(KeyLocation::new(KeySection::Cursor, 1, Key::CursorDown)),
            // Cursor left
            (0xE0, Some(0x4B)) => Some(KeyLocation::new(KeySection::Cursor, 2, Key::CursorLeft)),
            // Cursor right
            (0xE0, Some(0x4D)) => Some(KeyLocation::new(KeySection::Cursor, 3, Key::CursorRight)),
            _ => None
        }
    }
}

#[derive(Debug)]
pub struct KeyboardModel<const SCANCODE_CAP: usize> {
    scancode_queue: Queue<u8, SCANCODE_CAP>,
    extended_prefix: Option<u8>,
    modifiers: KeyModifiers,
}

impl<const SCANCODE_CAP: usize> KeyboardModel<SCANCODE_CAP> {
    pub const fn new() -> Self {
        Self { scancode_queue: Queue::new(), extended_prefix: None, modifiers: KeyModifiers::new() }
    }
}

impl<const SCANCODE_CAP: usize> Ps2Model for KeyboardModel<SCANCODE_CAP> {
    fn scancode_queue_capacity(&self) -> usize {
        SCANCODE_CAP
    }

    fn scancode_queue_len(&self) -> usize {
        self.scancode_queue.len()
    }

    fn push_scancode(&mut self, scancode: u8) -> Result<(), Ps2ModelError> {
        if self.scancode_queue.is_full() {
            return Err(Ps2ModelError::ScancodeQueueFull);
        }
        self.scancode_queue.enqueue(scancode).unwrap();
        Ok(())
    }

    fn pop_input(&mut self) -> Result<Option<HumanInput>, Ps2ModelError> {
        let mut scancode = if let Some(scancode) = self.scancode_queue.dequeue() {
            if let Some(prefix) = self.extended_prefix.take() {
                Scancode::Extended(prefix, scancode)
            } else {
                Scancode::Single(scancode)
            }
        } else {
            return Ok(None);
        };

        if scancode.is_single() && scancode.is_set_1_extended_prefix() {
            if self.scancode_queue.is_empty() {
                self.extended_prefix = Some(scancode.first_scancode());
                return Ok(None);
            }
            let next_scancode = self.scancode_queue.dequeue().unwrap();
            scancode = Scancode::Extended(scancode.first_scancode(), next_scancode);
        }

        let modifiers = self.modifiers;
        let location = match scancode.set_1_us_location() {
            Some(location) => location,
            None => {
                // match scancode {
                //     Scancode::Single(scancode) => crate::kprintln!("Unrecognised scancode: {:#02x}", scancode),
                //     Scancode::Extended(prefix, scancode) => crate::kprintln!("Unrecognised scancode: {:#02x}, {:#02x}", prefix, scancode),
                // }
                return Ok(None);
            }
        };
        let pressed = scancode.pressed();

        match (location.section(), location.key()) {
            (_, Key::CapsLock) => { self.modifiers = modifiers.set_caps_lock(pressed) }
            (KeySection::LeftModifiers, Key::Shift) => { self.modifiers = modifiers.set_left_shift(pressed) }
            (KeySection::LeftModifiers, Key::Control) => { self.modifiers = modifiers.set_left_control(pressed) }
            (KeySection::LeftModifiers, Key::Alt) => { self.modifiers = modifiers.set_left_alt(pressed) }
            (KeySection::RightModifiers, Key::Shift) => { self.modifiers = modifiers.set_right_shift(pressed) }
            (KeySection::RightModifiers, Key::Control) => { self.modifiers = modifiers.set_right_control(pressed) }
            (KeySection::RightModifiers, Key::Alt) => { self.modifiers = modifiers.set_right_alt(pressed) }
            _ => {}
        }

        let key = KeyState::new(modifiers, location, pressed);
        Ok(Some(HumanInput::Key(key)))
    }

    fn clear(&mut self) {
        self.scancode_queue = Queue::new();
    }
}
