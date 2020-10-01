use core::{future::Future, pin::Pin, fmt};
use core::task::{Context, Poll};
use core::sync::atomic::{AtomicU64, Ordering};
use alloc::boxed::Box;

pub mod executor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct TaskId(u64);

impl TaskId {
    fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        TaskId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

pub struct Task<'a> {
    id: TaskId,
    future: Pin<Box<dyn Future<Output=()> + 'a>>,
}

impl<'a> fmt::Debug for Task<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Task id:{:?}", self.id)
    }
}

impl<'a> Task<'a> {
    pub fn new(future: impl Future<Output=()> + 'a) -> Task<'a> {
        Task {
            id: TaskId::new(),
            future: Box::pin(future),
        }
    }

    fn poll(&mut self, context: &mut Context) -> Poll<()> {
        self.future.as_mut().poll(context)
    }
}
