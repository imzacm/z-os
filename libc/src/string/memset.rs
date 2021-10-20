use core::ptr;
use core::ffi::c_void;

pub unsafe fn memset_rs(buf_ptr: *mut u8, value: u8, size: usize) -> *mut u8 {
    ptr::write_bytes(buf_ptr, value, size);
    buf_ptr
}

#[no_mangle]
pub unsafe extern "C" fn memset(buf_ptr: *mut c_void, value: u8, size: usize) -> *mut c_void {
    memset_rs(buf_ptr as *mut u8, value, size);
    buf_ptr
}
