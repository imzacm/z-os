pub fn strlen_rs(str: *const u8) -> usize {
    let mut ptr = str;
    while unsafe { *ptr } != b'\0' {
        ptr = unsafe { ptr.add(1) };
    }
    (ptr as usize) - (str as usize)
}

#[no_mangle]
pub extern "C" fn strlen(str: *const u8) -> usize {
    strlen_rs(str)
}
