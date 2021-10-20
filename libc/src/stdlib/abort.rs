pub fn abort_rs() -> ! {
    #[allow(clippy::empty_loop)]
    loop {}
}

#[no_mangle]
pub extern "C" fn abort() -> ! {
    abort_rs()
}
