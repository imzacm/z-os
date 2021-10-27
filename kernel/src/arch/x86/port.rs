#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Port(u16);

#[allow(dead_code)]
impl Port {
    pub const fn new(port: u16) -> Self {
        Self(port)
    }

    /// outb
    pub unsafe fn write_u8(&self, value: u8) {
        asm!("out dx, al", in("dx") self.0, in("al") value, options(nomem, nostack, preserves_flags));
    }

    /// inb
    pub unsafe fn read_u8(&self) -> u8 {
        let value: u8;
        asm!("in al, dx", out("al") value, in("dx") self.0, options(nomem, nostack, preserves_flags));
        value
    }

    /// outw
    pub unsafe fn write_u16(&self, value: u16) {
        asm!("out dx, ax", in("dx") self.0, in("ax") value, options(nomem, nostack, preserves_flags));
    }

    /// inw
    pub unsafe fn read_u16(&self) -> u16 {
        let value: u16;
        asm!("in eax, dx", out("eax") value, in("dx") self.0, options(nomem, nostack, preserves_flags));
        value
    }

    /// outl
    pub unsafe fn write_u32(&self, value: u32) {
        asm!("out dx, eax", in("dx") self.0, in("eax") value, options(nomem, nostack, preserves_flags));
    }

    /// inl
    pub unsafe fn read_u32(&self) -> u32 {
        let value: u32;
        asm!("in eax, dx", out("eax") value, in("dx") self.0, options(nomem, nostack, preserves_flags));
        value
    }
}

pub unsafe fn io_wait() {
    const WAIT_PORT: Port = Port::new(0x80);
    WAIT_PORT.write_u8(0);
}
