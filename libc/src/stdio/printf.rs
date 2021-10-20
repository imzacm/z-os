use super::{EOF, putchar_rs};
use crate::string::strlen_rs;

fn print(data: *const u8, len: usize) -> bool {
    for i in 0..len {
        let c = unsafe { *data.add(i) };
        if putchar_rs(c as i32) == EOF {
            return false;
        }
    }
    true
}

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
pub unsafe extern "C" fn printf(format: *const u8, mut args: ...) -> i32 {
    let mut written: i32 = 0;

    let mut format_ptr = format;
    loop {
        let max_rem = i32::MAX - written;

        let format_0 = &*format_ptr;
        let format_1 = &*format_ptr.add(1);
        if format_0 != b'%' || format_1 == b'%' {

        }
        todo!()
    }

    written
}
