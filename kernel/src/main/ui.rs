// use lazy_static::lazy_static;
use z_os::task::executor::Executor;
// use z_os::task::Task;
// use z_os::hlt_loop;
// use futures_util::stream::Stream;
// use core::pin::Pin;
// use core::task::{Context, Poll};
// use futures_util::StreamExt;
// use spin::RwLock;
//
// use z_os::drivers::keyboard::driver::KeyPress;
// use z_os::drivers::keyboard::ps2::Ps2Keyboard;
//
// lazy_static! {
//     static ref KEYBOARD: RwLock<Ps2Keyboard> = {
//         RwLock::new(Ps2Keyboard::new())
//     };
// }
//
// async fn process_keypress(keypress: &KeyPress) {
//     z_os::serial_println!("Keypress: {:?}", keypress);
// }
//
// async fn await_keyboard_stream() {
//     let mut keyboard = KEYBOARD.write();
//     while let Some(keypress) = keyboard.next().await {
//         process_keypress(&keypress).await;
//     }
// }

pub fn enter_ui() -> ! {
    // unsafe {
    //     KEYBOARD.write().init().unwrap();
    // }
    let mut executor = Executor::new();
    // executor.spawn(Task::new(await_keyboard_stream()));
    executor.run()
}
