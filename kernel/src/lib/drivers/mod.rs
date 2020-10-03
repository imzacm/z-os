pub use core::fmt::Debug;
pub use alloc::boxed::Box;
use dyn_clone::{DynClone, clone_box};

pub mod traits;
pub mod keyboard;

pub trait DriverErrorTrait: Debug + DynClone {}

pub type DriverError = Box<dyn DriverErrorTrait>;

impl Clone for Box<dyn DriverErrorTrait> {
    fn clone(&self) -> Self {
        clone_box(self.as_ref())
    }
}

#[derive(Debug, Clone)]
pub enum DriverStatus {
    Stopped,
    Working,
    SlightlyBroken(DriverError),
    FullyBroken(DriverError),
}

pub trait Driver: Debug {
    fn get_status(&self) -> DriverStatus;

    fn init(&mut self) -> Result<(), DriverError>;

    fn de_init(&mut self) -> Result<(), DriverError>;

    fn enable(&mut self) -> Result<(), DriverError>;

    fn disable(&mut self) -> Result<(), DriverError>;
}

pub fn init_drivers() {}
