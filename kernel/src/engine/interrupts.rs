#[derive(Debug, Copy, Clone, Eq, PartialEq, hash32_derive::Hash32)]
pub struct Irq(u8);

impl Irq {
    pub const fn new(irq: u8) -> Self {
        Self(irq)
    }

    pub const fn as_u8(self) -> u8 {
        self.0
    }
}

pub type IrqListenerFn = fn(irq: Irq);

struct IrqListener {
    listener: IrqListenerFn,
}

use conquer_once::spin::OnceCell;
use spin::Mutex;
use heapless::FnvIndexMap;

const IRQ_LISTENER_SIZE: usize = 10;

static LISTENERS: OnceCell<Mutex<FnvIndexMap<Irq, IrqListener, IRQ_LISTENER_SIZE>>> = OnceCell::uninit();

fn get_listeners() -> &'static Mutex<FnvIndexMap<Irq, IrqListener, IRQ_LISTENER_SIZE>> {
    LISTENERS.get_or_init(|| Mutex::new(FnvIndexMap::new()))
}

pub fn request_interrupt(irq: Irq, listener: IrqListenerFn) {
    let listener = IrqListener { listener };
    get_listeners().lock().insert(irq, listener).ok();
}

pub fn handle_interrupt(interrupt: Irq) {
    // crate::kprintln!("Interrupt: {:?}", interrupt);
    if let Some(listeners) = LISTENERS.get() {
        unsafe { listeners.force_unlock() };
        let listener = listeners.lock()
            .get(&interrupt)
            .map(|l| l.listener);
        if let Some(listener) = listener {
            listener(interrupt);
        }
    }
}
