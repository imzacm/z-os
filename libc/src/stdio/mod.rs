mod puts;
mod putchar;
mod printf;

pub use puts::*;
pub use putchar::*;
pub use printf::*;

#[no_mangle]
pub static EOF: i32 = -1;
