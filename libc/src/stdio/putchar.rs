use core::mem;

#[cfg(feature = "kernel")]
extern "C" {
    fn terminal_write(data: *const u8, size: usize);
}

#[cfg(feature = "kernel")]
pub fn putchar_rs(c: i32) -> i32 {
    unsafe {
        let ptr = &c as *const i32 as *const u8;
        let size = mem::size_of_val(&c);
        terminal_write(ptr, size);
    }
    c
}

#[cfg(not(feature = "kernel"))]
pub fn putchar_rs(c: i32) -> i32 {
    todo!()
}
