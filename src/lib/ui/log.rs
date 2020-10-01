use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use futures_util::task::AtomicWaker;
use core::{pin::Pin, task::{Poll, Context}};
use core::fmt;
use futures_util::stream::{Stream, StreamExt};
use pc_keyboard::{Keyboard, layouts, ScancodeSet1, HandleControl, DecodedKey, KeyboardLayout, ScancodeSet};
use crate::serial_println;
use alloc::string::String;

static WAKER: AtomicWaker = AtomicWaker::new();

static LOG_QUEUE: OnceCell<ArrayQueue<String>> = OnceCell::uninit();

pub struct Logger;

impl Logger {
    pub fn new() -> Logger {
        LOG_QUEUE.try_init_once(|| ArrayQueue::new(100))
            .expect("Logger::new should only be called once");

        Logger {}
    }
}

impl Stream for Logger {
    type Item = String;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<String>> {
        let queue = LOG_QUEUE
            .try_get()
            .expect("Scancode queue not initialized");

        if let Ok(log_args) = queue.pop() {
            return Poll::Ready(Some(log_args));
        }

        WAKER.register(&cx.waker());
        match queue.pop() {
            Ok(log_args) => {
                WAKER.take();
                Poll::Ready(Some(log_args))
            }
            Err(crossbeam_queue::PopError) => Poll::Pending,
        }
    }
}

impl Logger {
    pub async fn stream_logs<T: Fn(String)>(&mut self, stream_cb: T) {
        while let Some(log_args) = self.next().await {
            stream_cb(log_args);
        }
    }
}

pub fn push_log(log_args: String) {
    if let Ok(queue) = LOG_QUEUE.try_get() {
        if let Err(_) = queue.push(log_args) {
            serial_println!("WARNING: Log queue full; dropping keyboard input");
        } else {
            WAKER.wake();
        }
    } else {
        serial_println!("WARNING: Log queue uninitialized");
    }
}
