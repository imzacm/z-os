mod structures;
mod commands;
mod models;

use structures::*;
use models::*;

// struct ControllerState<const DEVICE_1_CAP: usize, const DEVICE_2_CAP: usize> {
//     device_1: Option<Ps2DeviceModel<DEVICE_1_CAP>>,
//     device_2: Option<Ps2DeviceModel<DEVICE_2_CAP>>,
// }
//
// impl<const DEVICE_1_CAP: usize, const DEVICE_2_CAP: usize> ControllerState<DEVICE_1_CAP, DEVICE_2_CAP> {
//     fn new() -> Self {
//         Self { device_1: None, device_2: None }
//     }
//
//     fn detect_device(&mut self, port: commands::Ps2Port) {
//         use commands::*;
//
//         let device_type = enable_port(port)
//             .and_then(|_| reset_device(port))
//             .and_then(|_| disable_scanning(port))
//             .and_then(|_| identify_device(port));
//
//         match device_type {
//             Ok(device_type) if port == Ps2Port::First => {
//                 self.device_1 = Some(device_type.into());
//             }
//             Ok(device_type) if port == Ps2Port::Second => {
//                 self.device_1 = Some(device_type.into());
//             }
//             _ if port == Ps2Port::First => {
//                 self.device_1 = None;
//             }
//             _ if port == Ps2Port::Second => {
//                 self.device_2 = None;
//             }
//             _ => unreachable!()
//         }
//     }
//
//     fn detect_device_1(&mut self) -> Option<&mut Ps2DeviceModel<DEVICE_1_CAP>> {
//         self.detect_device(commands::Ps2Port::First);
//         self.device_1.as_mut()
//     }
//
//     fn detect_device_2(&mut self) -> Option<&mut Ps2DeviceModel<DEVICE_2_CAP>> {
//         self.detect_device(commands::Ps2Port::Second);
//         self.device_2.as_mut()
//     }
//
//     fn is_device_connected(&self, port: commands::Ps2Port) -> bool {
//         match port {
//             commands::Ps2Port::First => self.device_1.is_some(),
//             commands::Ps2Port::Second => self.device_2.is_some(),
//         }
//     }
//
//     fn push_scancode(&mut self, port: commands::Ps2Port, scancode: u8) -> Result<(), Ps2ModelError> {
//         use commands::Ps2Port;
//
//         match (port, self.device_1.as_mut(), self.device_2.as_mut()) {
//             (Ps2Port::First, Some(model), _) => model.push_scancode(scancode),
//             (Ps2Port::Second, _, Some(model)) => model.push_scancode(scancode),
//             _ => Ok(())
//         }
//     }
//
//     fn pop_input(&mut self, port: commands::Ps2Port) -> Result<Option<crate::engine::human_input::HumanInput>, Ps2ModelError> {
//         use commands::Ps2Port;
//
//         match (port, self.device_1.as_mut(), self.device_2.as_mut()) {
//             (Ps2Port::First, Some(model), _) => model.pop_input(),
//             (Ps2Port::Second, _, Some(model)) => model.pop_input(),
//             _ => Ok(None)
//         }
//     }
// }
//
// use conquer_once::spin::OnceCell;
// use spin::Mutex;
//
// const DEVICE_1_CAP: usize = 3;
// const DEVICE_2_CAP: usize = 3;
// static CONTROLLER_STATE: OnceCell<Mutex<ControllerState<DEVICE_1_CAP, DEVICE_2_CAP>>> = OnceCell::uninit();
//
// pub fn init_ps2() -> bool {
//     use commands::*;
//
//     macro_rules! check_error {
//         ($result:expr) => {
//             match $result {
//                 Ok(result) => result,
//                 Err(_) => return false
//             }
//         }
//     }
//
//     check_error!(disable_port(Ps2Port::First));
//     check_error!(disable_port(Ps2Port::Second));
//     flush_output_buffer();
//
//     let config = check_error!(read_controller_config())
//         .set_first_port_interrupt(false)
//         .set_second_port_interrupt(false)
//         .set_first_port_translation(false);
//     check_error!(write_controller_config(config));
//     check_error!(test_controller());
//     check_error!(write_controller_config(config));
//
//     check_error!(enable_port(Ps2Port::Second));
//     let config = check_error!(read_controller_config())
//         .set_first_port_interrupt(false)
//         .set_second_port_interrupt(false)
//         .set_first_port_translation(false);
//     let has_second_port = config.second_port_clock();
//     check_error!(disable_port(Ps2Port::Second));
//
//     check_error!(test_port(Ps2Port::First));
//     let has_second_port = has_second_port && test_port(Ps2Port::Second).is_ok();
//
//     let config = check_error!(read_controller_config())
//         .set_first_port_interrupt(true)
//         .set_second_port_interrupt(true)
//         .set_first_port_translation(true);
//     check_error!(write_controller_config(config));
//
//     {
//         let mut controller_state = ControllerState::new();
//         controller_state.detect_device_1();
//         if has_second_port { controller_state.detect_device_2(); }
//         CONTROLLER_STATE.init_once(|| Mutex::new(controller_state));
//     }
//
//     crate::engine::request_interrupt(crate::engine::Irq::new(1), handle_ps2_irq);
//     crate::engine::request_interrupt(crate::engine::Irq::new(12), handle_ps2_irq);
//
//     true
// }

// fn handle_ps2_irq(irq: crate::engine::Irq) {
//     let controller_state = CONTROLLER_STATE.get()
//         .expect("handle_ps2_irq was called before init_ps2 or init_ps2 failed");
//     let scancode = unsafe { commands::read_byte_unchecked() };
//
//     let device_port = match irq.as_u8() {
//         1 => commands::Ps2Port::First,
//         12 => commands::Ps2Port::Second,
//         _ => panic!("handle_ps2_irq was called with Irq({}), expected Irq(1) or Irq(12)", irq.as_u8())
//     };
//
//     let mut controller_state_lock = controller_state.lock();
//     if !controller_state_lock.is_device_connected(device_port) {
//         controller_state_lock.detect_device(device_port);
//         if !controller_state_lock.is_device_connected(device_port) {
//             return;
//         }
//     }
//     controller_state_lock.push_scancode(device_port, scancode).unwrap();
//     let input = match controller_state_lock.pop_input(device_port) {
//         Ok(Some(input)) => input,
//         _ => return
//     };
//     todo!("Handle input")
// }

use conquer_once::spin::OnceCell;
use spin::Mutex;

const SCANCODE_CAP: usize = 3;
static DEVICE_1_MODEL: OnceCell<Mutex<Ps2DeviceModel<SCANCODE_CAP>>> = OnceCell::uninit();

pub fn init_ps2() -> bool {
    DEVICE_1_MODEL.init_once(|| Mutex::new(Ps2DeviceModel::from(Ps2DeviceType::AtKeyboard)));
    crate::engine::request_interrupt(crate::engine::Irq::new(1), handle_ps2_irq);
    true
}

fn handle_ps2_irq(_irq: crate::engine::Irq) {
    let model = DEVICE_1_MODEL.get().unwrap();

    let scancode = unsafe { commands::read_byte_unchecked() };
    let mut model_lock = model.lock();
    model_lock.push_scancode(scancode).ok();

    while let Ok(Some(input)) = model_lock.pop_input() {
        crate::kprintln!("Keyboard input: {:?}", input);
    }
}
