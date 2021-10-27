mod port;
mod tty;
mod gdt;
mod idt;
mod pic;

pub(in crate::arch) use port::{Port, io_wait};
pub(in crate::arch) use idt::InterruptStackFrame;

#[repr(C, packed)]
pub(in crate::arch) struct TablePointer {
    limit: u16,
    base: u32,
}

impl TablePointer {
    pub fn new<T, const LEN: usize>(table: &[T; LEN]) -> Self {
        let limit = (core::mem::size_of::<T>() * LEN) - 1;
        let base = table.as_ptr() as usize;
        Self { limit: limit as u16, base: base as u32 }
    }
}

#[no_mangle]
extern "C" fn x86_kernel_entry() {
    unsafe {
        crate::kprintln!("Setting up GDT...");
        gdt::setup_gdt();
        crate::kprintln!("Setting up IDT...");
        idt::setup_idt();
        crate::kprintln!("Setting up PIC...");
        pic::setup_pic();
        crate::kprintln!("Enabling interrupts...");
        enable_interrupts();
    }
}

pub(in crate::arch) fn get_tty() -> impl core::fmt::Write {
    struct Writer;

    impl core::fmt::Write for Writer {
        fn write_str(&mut self, s: &str) -> core::fmt::Result {
            // without_interrupts(|| {
            //
            // })
            let tty = tty::get_tty();
            tty.lock().write_str(s);
            Ok(())
        }
    }

    Writer
}

pub fn idle() -> ! {
    unsafe {
        asm!("hlt");
        core::intrinsics::unreachable();
    }
}

pub fn interrupts_enabled() -> bool {
    // TODO: Move eflags into it's own structure
    let eflags: u32;
    unsafe { asm!("pushfd; pop {}", out(reg) eflags, options(nomem, preserves_flags)) };
    let interrupts_enabled_bit = 0x0200 & (1 << eflags);
    interrupts_enabled_bit != 0
}

pub fn disable_interrupts() {
    unsafe { asm!("cli") };
}

pub fn enable_interrupts() {
    unsafe { asm!("sti") };
}

pub fn without_interrupts<R, F: FnMut() -> R>(mut f: F) -> R {
    let enabled = interrupts_enabled();
    if !enabled {
        return f();
    }
    disable_interrupts();
    let result = f();
    enable_interrupts();
    result
}
