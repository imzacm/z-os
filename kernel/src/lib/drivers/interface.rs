use conquer_once::spin::{OnceCell, Once};
use crossbeam_queue::ArrayQueue;
use core::result::Result;
use conquer_once::TryGetError;
use futures_util::task::AtomicWaker;
use lazy_static::lazy_static;
use spin::RwLock;

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();

lazy_static! {
    pub static ref KEYBOARD_WAKER: RwLock<AtomicWaker> = RwLock::new(AtomicWaker::new());
}

pub fn init_drivers() {
    SCANCODE_QUEUE.try_init_once(|| ArrayQueue::new(100))
        .expect("Failed to initialise scancode queue");
}

pub fn read_keyboard_scancode() -> Result<Option<u8>, TryGetError> {
    let queue = SCANCODE_QUEUE
        .try_get()?;

    if let Ok(scancode) = queue.pop() {
        return Ok(Some(scancode));
    }
    Ok(None)
}

pub fn push_keyboard_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if let Err(_) = queue.push(scancode) {
            // println!("WARNING: scancode queue full; dropping keyboard input");
        } else {
            KEYBOARD_WAKER.read().wake();
        }
    } else {
        // println!("WARNING: scancode queue uninitialized");
    }
}
