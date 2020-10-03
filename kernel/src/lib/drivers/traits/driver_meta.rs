use core::fmt::Debug;

pub trait DriverMeta where Self: Debug {
    fn get_version(&self) -> &'static str;

    fn get_name(&self) -> &'static str;

    fn get_description(&self) -> &'static str;
}
