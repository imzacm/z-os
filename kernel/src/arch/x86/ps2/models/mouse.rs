use super::{Ps2Model, Ps2ModelError};
use crate::engine::human_input::HumanInput;
use heapless::spsc::Queue;

#[derive(Debug)]
pub struct MouseModel<const SCANCODE_CAP: usize> {
    scancode_queue: Queue<u8, SCANCODE_CAP>,
}

impl<const SCANCODE_CAP: usize> MouseModel<SCANCODE_CAP> {
    pub const fn new() -> Self {
        Self { scancode_queue: Queue::new() }
    }
}

impl<const SCANCODE_CAP: usize> Ps2Model for MouseModel<SCANCODE_CAP> {
    fn scancode_queue_capacity(&self) -> usize {
        SCANCODE_CAP
    }

    fn scancode_queue_len(&self) -> usize {
        self.scancode_queue.len()
    }

    fn push_scancode(&mut self, scancode: u8) -> Result<(), Ps2ModelError> {
        if self.scancode_queue.is_full() {
            return Err(Ps2ModelError::ScancodeQueueFull)
        }
        self.scancode_queue.enqueue(scancode).unwrap();
        Ok(())
    }

    fn pop_input(&mut self) -> Result<Option<HumanInput>, Ps2ModelError> {
        todo!()
    }

    fn clear(&mut self) {
        self.scancode_queue = Queue::new();
    }
}
