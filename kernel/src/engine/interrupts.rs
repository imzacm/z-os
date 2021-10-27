#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Interrupt {
    Timer,
}

// TODO: Add interrupt detail
pub fn handle_interrupt(interrupt: Interrupt) {
    crate::kprintln!("Interrupt: {:?}", interrupt);
}
