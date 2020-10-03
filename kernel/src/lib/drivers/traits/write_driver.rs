use core::fmt::Debug;
use futures_util::task::AtomicWaker;
use crate::drivers::traits::driver_meta::DriverMeta;

#[derive(Debug, Copy, Clone)]
pub enum DriverWriteError {}

pub trait Writable<T> where Self: Debug {
    fn write_ref(&mut self, value: &T) -> Result<(), DriverWriteError>;

    fn write(&mut self, value: T) -> Result<(), DriverWriteError>;
}

pub trait WriteDriver<T>: DriverMeta + Writable<T> {}
