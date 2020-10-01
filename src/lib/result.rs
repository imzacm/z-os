use core::result::Result;
use core::fmt::Debug;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::format;

pub trait OsError where Self: Debug {
    fn debug_info(&self) -> String;

    fn user_message(&self) -> &'static str;

    fn dev_message(&self) -> &'static str;
}

#[derive(Debug, Clone)]
pub struct GenericOsError<T: Debug> {
    debug: T,
    user_msg: &'static str,
    dev_msg: &'static str,
}

impl <T: Debug>GenericOsError<T> {
    pub fn new(debug: T, user_msg: &'static str, dev_msg: &'static str) -> GenericOsError<T> {
        GenericOsError {
            debug,
            user_msg,
            dev_msg
        }
    }
}

impl <T: Debug>OsError for GenericOsError<T> {
    fn debug_info(&self) -> String {
        format!("{:?}", self.debug)
    }

    fn user_message(&self) -> &'static str {
        self.user_msg
    }

    fn dev_message(&self) -> &'static str {
        self.dev_msg
    }
}

pub type OsResult<T, E = Box<dyn OsError>> = Result<T, E>;
