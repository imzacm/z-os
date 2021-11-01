use crate::engine::{Irq, request_interrupt};
use conquer_once::spin::OnceCell;
use spin::Mutex;

static COUNTER: OnceCell<Mutex<u8>> = OnceCell::uninit();

pub fn init_pit_timer() {
    COUNTER.init_once(|| Mutex::new(0));
    request_interrupt(Irq::new(0), handle_timer_irq);
}

fn handle_timer_irq(_irq: Irq) {
    let counter = COUNTER.get()
        .expect("handle_timer_irq called before init_pit_timer");
    let mut counter_lock = counter.lock();
    *counter_lock += 1;
    if *counter_lock == 18 {
        // crate::kprintln!("Timer: 1 second");
        *counter_lock = 0;
    }
}
