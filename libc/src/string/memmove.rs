use core::ptr;
use core::ffi::c_void;

pub unsafe fn memmove_rs(dst: *mut u8, src: *const u8, size: usize) -> *mut u8 {
    ptr::copy(src, dst, size);
    dst
}

#[no_mangle]
pub unsafe extern "C" fn memmove(dst: *mut c_void, src: *const c_void, size: usize) -> *mut c_void {
    memmove_rs(dst as *mut u8, src as *const u8, size);
    dst
}
