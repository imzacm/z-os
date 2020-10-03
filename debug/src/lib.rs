#![no_std]
use uart_16550::SerialPort;
use spin::Mutex;
use lazy_static::lazy_static;
use x86_64::instructions::interrupts;

lazy_static! {
    static ref SERIAL1: Mutex<SerialPort> = {
        let mut serial_port = unsafe { SerialPort::new(0x3F8) };
        serial_port.init();
        Mutex::new(serial_port)
    };
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;

    interrupts::without_interrupts(|| {
        SERIAL1
            .lock()
            .write_fmt(args)
            .expect("Printing to serial failed");
    });
}

#[macro_export]
macro_rules! debug_print {
    ($($arg:tt)*) => {
        $crate::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! debug {
    () => ($crate::debug_print!("\n"));
    ($fmt:expr) => ($crate::debug_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::debug_print!(
        concat!($fmt, "\n"), $($arg)*));
}
