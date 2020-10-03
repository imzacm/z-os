use core::fmt::Debug;
use futures_util::task::AtomicWaker;
use spin::RwLock;
use crate::drivers::traits::driver_meta::DriverMeta;

#[derive(Debug, Copy, Clone)]
pub enum DriverReadError {
    Uninitialised
}

pub trait Readable<T> where Self: Debug {
    fn get_waker(&mut self) -> &RwLock<AtomicWaker>;

    fn read(&mut self) -> Result<Option<T>, DriverReadError>;
}

pub trait ReadDriver<T>: DriverMeta + Readable<T> {}
