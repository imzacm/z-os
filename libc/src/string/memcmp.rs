use core::ffi::c_void;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
#[repr(i8)]
pub enum CmpResult {
    LessThan = -1,
    GreaterThan = 1,
    Equal = 0,
}

pub unsafe fn memcmp_rs(a_ptr: *const u8, b_ptr: *const u8, size: usize) -> CmpResult {
    for i in 0..size {
        let a = &*a_ptr.add(i);
        let b = &*b_ptr.add(i);
        if a < b {
            return CmpResult::LessThan;
        }
        if a > b {
            return CmpResult::GreaterThan;
        }
    }
    CmpResult::Equal
}

#[no_mangle]
pub unsafe extern "C" fn memcmp(a_ptr: *const c_void, b_ptr: *const c_void, size: usize) -> i8 {
    memcmp_rs(a_ptr as *const u8, b_ptr as *const u8, size) as i8
}
