use core::fmt;

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KeyModifiers(u8);

impl KeyModifiers {
    const CAPS_LOCK: u8 = 1 << 0;
    const LEFT_SHIFT: u8 = 1 << 1;
    const LEFT_CONTROL: u8 = 1 << 2;
    const LEFT_ALT: u8 = 1 << 3;
    const RIGHT_SHIFT: u8 = 1 << 4;
    const RIGHT_CONTROL: u8 = 1 << 5;
    const RIGHT_ALT: u8 = 1 << 6;

    pub const fn new() -> Self {
        Self(0)
    }

    pub const fn from_u8(value: u8) -> Self {
        Self(value)
    }

    pub const fn as_u8(self) -> u8 {
        self.0
    }

    pub const fn caps_lock(self) -> bool {
        (self.0 & Self::CAPS_LOCK) == 1
    }

    pub const fn set_caps_lock(self, enabled: bool) -> Self {
        let mut value = self.0;
        if enabled { value |= Self::CAPS_LOCK } else { value &= !(Self::CAPS_LOCK) };
        Self(value)
    }

    pub const fn left_shift(self) -> bool {
        (self.0 & Self::LEFT_SHIFT) == 1
    }

    pub const fn set_left_shift(self, enabled: bool) -> Self {
        let mut value = self.0;
        if enabled { value |= Self::LEFT_SHIFT } else { value &= !(Self::LEFT_SHIFT) };
        Self(value)
    }

    pub const fn left_control(self) -> bool {
        (self.0 & Self::LEFT_CONTROL) == 1
    }

    pub const fn set_left_control(self, enabled: bool) -> Self {
        let mut value = self.0;
        if enabled { value |= Self::LEFT_CONTROL } else { value &= !(Self::LEFT_CONTROL) };
        Self(value)
    }

    pub const fn left_alt(self) -> bool {
        (self.0 & Self::LEFT_ALT) == 1
    }

    pub const fn set_left_alt(self, enabled: bool) -> Self {
        let mut value = self.0;
        if enabled { value |= Self::LEFT_ALT } else { value &= !(Self::LEFT_ALT) };
        Self(value)
    }

    pub const fn right_shift(self) -> bool {
        (self.0 & Self::RIGHT_SHIFT) == 1
    }

    pub const fn set_right_shift(self, enabled: bool) -> Self {
        let mut value = self.0;
        if enabled { value |= Self::RIGHT_SHIFT } else { value &= !(Self::RIGHT_SHIFT) };
        Self(value)
    }

    pub const fn right_control(self) -> bool {
        (self.0 & Self::RIGHT_CONTROL) == 1
    }

    pub const fn set_right_control(self, enabled: bool) -> Self {
        let mut value = self.0;
        if enabled { value |= Self::RIGHT_CONTROL } else { value &= !(Self::RIGHT_CONTROL) };
        Self(value)
    }

    pub const fn right_alt(self) -> bool {
        (self.0 & Self::RIGHT_ALT) == 1
    }

    pub const fn set_right_alt(self, enabled: bool) -> Self {
        let mut value = self.0;
        if enabled { value |= Self::RIGHT_ALT } else { value &= !(Self::RIGHT_ALT) };
        Self(value)
    }
}

impl fmt::Debug for KeyModifiers {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("KeyModifiers")
            .field("caps_lock", &self.caps_lock())
            .field("left_shift", &self.left_shift())
            .field("left_alt", &self.left_alt())
            .field("left_control", &self.left_control())
            .field("right_shift", &self.right_shift())
            .field("right_alt", &self.right_alt())
            .field("right_control", &self.right_control())
            .finish()
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum KeySection {
    /// Escape gets it's own section
    Escape = 0,
    /// F1 to F12
    Function = 1,
    /// Print screen, Scroll lock, Pause/Break, Insert, Delete,
    /// Home, End, Page up, and Page down
    Special = 2,
    /// TODO
    Media = 3,
    /// Arrow keys (Up, Down, Left, Right)
    Cursor = 4,
    /// NumLock, /, *, -, +, 0 to 9, Delete, and Enter
    Keypad = 5,
    /// Caps lock, Shift, Control, Super, and Alt
    LeftModifiers = 6,
    /// Shift, Alt, and Control
    RightModifiers = 7,
    /// All the rest (numbers, symbols, letters, Tab, Backspace, and Enter)
    Center = 8,
}

impl KeySection {
    pub const fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Escape,
            1 => Self::Function,
            2 => Self::Special,
            3 => Self::Media,
            4 => Self::Cursor,
            5 => Self::Keypad,
            6 => Self::LeftModifiers,
            7 => Self::RightModifiers,
            8 => Self::Center,
            // Can't put a message inside a const fn
            _ => panic!(),
        }
    }

    pub const fn as_u8(self) -> u8 {
        self as u8
    }
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Key {
    Escape = 0,
    F1 = 1,
    F2 = 2,
    F3 = 3,
    F4 = 4,
    F5 = 5,
    F6 = 6,
    F7 = 7,
    F8 = 8,
    F9 = 9,
    F10 = 10,
    F11 = 11,
    F12 = 12,
    PrintScreen = 13,
    NumLock = 14,
    ScrollLock = 15,
    PauseBreak = 16,
    Insert = 17,
    Delete = 18,
    Home = 19,
    End = 20,
    PageUp = 21,
    PageDown = 22,
    CursorUp = 23,
    CursorDown = 24,
    CursorLeft = 25,
    CursorRight = 26,
    CapsLock = 27,
    Shift = 28,
    Alt = 29,
    Control = 30,
    Super = 31,
    Num0 = 32,
    Num1 = 33,
    Num2 = 34,
    Num3 = 35,
    Num4 = 36,
    Num5 = 37,
    Num6 = 38,
    Num7 = 39,
    Num8 = 40,
    Num9 = 41,
    Tab = 42,
    Backspace = 43,
    Enter = 44,
    BackTick = 45,
    Hyphen = 46,
    Equals = 47,
    OpenCurly = 48,
    CloseCurly = 49,
    SemiColon = 50,
    SingleQuote = 51,
    Hash = 52,
    BackSlash = 53,
    Comma = 54,
    FullStop = 55,
    ForwardSlash = 56,
    Q = 57,
    W = 58,
    E = 59,
    R = 60,
    T = 61,
    Y = 62,
    U = 63,
    I = 64,
    O = 65,
    P = 66,
    A = 67,
    S = 68,
    D = 69,
    F = 70,
    G = 71,
    H = 72,
    J = 73,
    K = 74,
    L = 75,
    Z = 76,
    X = 77,
    C = 78,
    V = 79,
    B = 80,
    N = 81,
    M = 82,
    Space = 83,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct KeyLocation {
    section: KeySection,
    /// It's up to the KeyMap to decide what the index means
    index: u8,
    /// Key is what the Keyboard river thinks was pressed
    key: Key,
}

impl KeyLocation {
    pub const fn new(section: KeySection, index: u8, key: Key) -> Self {
        Self { section, index, key }
    }

    pub const fn section(self) -> KeySection {
        self.section
    }

    pub const fn index(self) -> u8 {
        self.index
    }

    pub const fn key(self) -> Key {
        self.key
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct KeyState {
    /// Modifiers at the time the key was pressed/released
    modifiers: KeyModifiers,
    location: KeyLocation,
    pressed: bool,
}

impl KeyState {
    pub const fn new(modifiers: KeyModifiers, location: KeyLocation, pressed: bool) -> Self {
        Self { modifiers, location, pressed }
    }

    pub const fn modifiers(self) -> KeyModifiers {
        self.modifiers
    }

    pub const fn location(self) -> KeyLocation {
        self.location
    }

    pub const fn pressed(self) -> bool {
        self.pressed
    }
}

pub trait KeyMap {
    /// Returns `None` if no character represents the location
    fn lookup_key(&self, location: KeyLocation) -> Option<char>;
}
