use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use futures_util::task::AtomicWaker;
use core::{pin::Pin, task::{Poll, Context}};
use core::fmt;
use futures_util::stream::{Stream, StreamExt};
use pc_keyboard::{Keyboard, layouts, ScancodeSet1, HandleControl, DecodedKey, KeyboardLayout, ScancodeSet};
use crate::serial_println;

static WAKER: AtomicWaker = AtomicWaker::new();
static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();

#[derive(Debug)]
pub struct Ps2Keyboard;

impl Ps2Keyboard {
    pub fn new() -> Ps2Keyboard {
        SCANCODE_QUEUE.try_init_once(|| ArrayQueue::new(100))
            .expect("Ps2Keyboard::new should only be called once");

        Ps2Keyboard
    }
}

impl Stream for Ps2Keyboard {
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
        let queue = SCANCODE_QUEUE
            .try_get()
            .expect("Scancode queue not initialized");

        if let Ok(scancode) = queue.pop() {
            return Poll::Ready(Some(scancode));
        }

        WAKER.register(&cx.waker());
        match queue.pop() {
            Ok(scancode) => {
                WAKER.take();
                Poll::Ready(Some(scancode))
            }
            Err(crossbeam_queue::PopError) => Poll::Pending,
        }
    }
}

impl Ps2Keyboard {
    pub async fn stream_key_events<T: Fn(DecodedKey)>(&mut self, stream_cb: T) {
        let mut keyboard = Keyboard::new(layouts::Uk105Key, ScancodeSet1, HandleControl::Ignore);
        while let Some(scancode) = self.next().await {
            if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
                if let Some(key) = keyboard.process_keyevent(key_event) {
                    stream_cb(key);
                }
            }
        }
    }
}

pub fn add_scancode(scancode: u8) {
    if let Ok(queue) = SCANCODE_QUEUE.try_get() {
        if let Err(_) = queue.push(scancode) {
            serial_println!("WARNING: Scancode queue full; dropping keyboard input");
        } else {
            WAKER.wake();
        }
    } else {
        serial_println!("WARNING: Scancode queue uninitialized");
    }
}
