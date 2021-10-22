mod io;
mod isrs;
mod tty;
mod gdt;

pub use tty::print;

extern "C" {
    // fn init_gdt();
}

pub fn init() {
    tty::init_tty();
    // unsafe { init_gdt() };
    unsafe { gdt::init_gdt() };
}
