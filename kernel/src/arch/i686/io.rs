#![allow(deprecated)]

pub unsafe fn outb(port: u16, value: u8) {
    llvm_asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(value));
}

pub unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    llvm_asm!("inb %dx, %al" : "={al}"(value) : "{dx}"(port) :: "volatile");
    value
}

pub unsafe fn outw(port: u16, value: u16) {
    llvm_asm!("outw %ax, %dx" :: "{dx}"(port), "{ax}"(value));
}

pub unsafe fn inw(port: u16) -> u16 {
    let value: u16;
    llvm_asm!("inw %dx, %ax" : "={ax}"(value) : "{dx}"(port) :: "volatile");
    value
}

pub unsafe fn outl(port: u16, value: u32) {
    llvm_asm!("outl %eax, %dx" :: "{dx}"(port), "{eax}"(value));
}

pub unsafe fn inl(port: u16) -> u32 {
    let value: u32;
    llvm_asm!("inl %dx, %eax" : "={eax}"(value) : "{dx}"(port) :: "volatile");
    value
}
