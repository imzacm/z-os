mod key;

pub use key::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum HumanInput {
    Key(KeyState),
    // Mouse,
}
