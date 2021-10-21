mod io;
mod gdt;
mod idt;
mod isrs;
mod tty;

pub use tty::{print, println};

pub fn init() {
    tty::init_tty();
}
