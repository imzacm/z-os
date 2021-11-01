mod key;

pub use key::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum HumanInput {
    Key(KeyState),
    // Mouse,
}

pub fn handle_human_input(input: HumanInput) {
    crate::kprintln!("Human input: {:?}", input);
}
