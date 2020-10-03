use pc_keyboard::{Keyboard, HandleControl, KeyboardLayout, ScancodeSet, layouts, ScancodeSet1, ScancodeSet2};
use crate::drivers::{Driver, DriverStatus, DriverError};
use x86_64::instructions::port::Port;
use spin::RwLock;
use lazy_static::lazy_static;

#[cfg(feature = "ps2_scancode_set_1")]
const SCANCODE: u16 = 0x1;

#[cfg(feature = "ps2_scancode_set_2")]
const SCANCODE: u16 = 0x2;

#[cfg(all(feature = "ps2_scancode_set_1", feature = "locale_uk"))]
lazy_static! {
    static ref KEYBOARD: RwLock<Keyboard<layouts::Uk105Key, ScancodeSet1>> =
        RwLock::new(Keyboard::new(layouts::Uk105Key, ScancodeSet1, HandleControl::Ignore));
}

#[cfg(all(feature = "ps2_scancode_set_2", feature = "locale_uk"))]
lazy_static! {
    static ref KEYBOARD: RwLock<Keyboard<layouts::Uk105Key, ScancodeSet2>> =
        RwLock::new(Keyboard::new(layouts::Uk105Key, ScancodeSet2, HandleControl::Ignore));
}

const DATA_PORT: u16 = 0x60;
const COMMAND_PORT: u16 = 0x64;
const STATUS_PORT: u16 = 0x64;

const SET_SCANCODE: (u16, u16) = (0xF0, SCANCODE);

#[derive(Debug)]
pub struct Ps2Keyboard {
    status: DriverStatus
}

impl Ps2Keyboard {
    pub fn new() -> Ps2Keyboard {
        Ps2Keyboard { status: DriverStatus::Stopped }
    }

    unsafe fn _set_scancode_set(&mut self) -> Result<(), DriverError> {
        let (command, data) = SET_SCANCODE;

        let mut command_port = Port::new(COMMAND_PORT);
        command_port.write(command);

        let mut data_port = Port::new(DATA_PORT);
        data_port.write(data);

        let response = data_port.read();
        match response {
            0xFA => {
                crate::debug!("Ack");
                Ok(())
            }
            0xFAFA => {
                crate::debug!("Ack? {:?}", response);
                Ok(())
            }
            0xFE => {
                crate::debug!("Resend");
                Ok(())
            }
            _ => {
                crate::debug!("Other: {:?}", response);
                Ok(())
            }
        }
    }
}

impl Driver for Ps2Keyboard {
    fn get_status(&self) -> DriverStatus {
        self.status.clone()
    }

    fn init(&mut self) -> Result<(), DriverError> {
        unsafe {
            self._set_scancode_set()?;
        }

        self.status = DriverStatus::Working;
        Ok(())
    }

    fn de_init(&mut self) -> Result<(), DriverError> {
        unimplemented!()
    }

    fn enable(&mut self) -> Result<(), DriverError> {
        unimplemented!()
    }

    fn disable(&mut self) -> Result<(), DriverError> {
        unimplemented!()
    }
}

// impl Ps2Keyboard {
//     pub unsafe fn init(&mut self) -> Result<(), DriverReadError> {
//         let mut port = Port::new(0x60);
//         port.write(0xF0 as u16);
//         port.write(0x2 as u16);
//         let response = port.read();
//         match response {
//             0xFA => {
//                 crate::serial_println!("Ack");
//             },
//             0xFAFA => {
//                 crate::serial_println!("Ack? {:?}", response);
//             },
//             0xFE => {
//                 crate::serial_println!("Resend");
//             },
//             _ => {
//                 crate::serial_println!("Other: {:?}", response);
//             }
//         }
//
//         Ok(())
//     }
// }
//
// impl DriverMeta for Ps2Keyboard {
//     fn get_version(&self) -> &'static str {
//         PS2_VERSION
//     }
//
//     fn get_name(&self) -> &'static str {
//         PS2_NAME
//     }
//
//     fn get_description(&self) -> &'static str {
//         PS2_DESCRIPTION
//     }
// }
//
// impl Readable<KeyPress> for Ps2Keyboard {
//     fn get_waker(&mut self) -> &RwLock<AtomicWaker> {
//         &KEYBOARD_WAKER
//     }
//
//     fn read(&mut self) -> Result<Option<KeyPress>, DriverReadError> {
//         let scancode = read_keyboard_scancode()
//             .map_err(|_e| DriverReadError::Uninitialised)?;
//
//         let scancode = match scancode {
//             Some(scancode) => scancode,
//             None => {
//                 return Ok(None)
//             }
//         };
//
//         let key_event = self.keyboard.add_byte(scancode)
//             .map_err(|_e|DriverReadError::Uninitialised)?;
//         let key_event = match key_event {
//             Some(key_event) => key_event,
//             None => {
//                 return Ok(None)
//             }
//         };
//
//         let modifiers = self.keyboard.get_modifiers().clone();
//
//         crate::serial_println!("KeyEvent: {:?}", key_event);
//
//         let decoded_key = self.keyboard.process_keyevent(key_event.clone());
//         let decoded_key = match decoded_key {
//             Some(decoded_key) => decoded_key,
//             None => {
//                 return Ok(None)
//             }
//         };
//
//         Ok(Some(KeyPress {
//             scan_code: scancode,
//             key_code: key_event.code,
//             key_state: key_event.state,
//             modifiers,
//             unicode: match decoded_key {
//                 DecodedKey::Unicode(key) => Some(key),
//                 _ => None
//             },
//         }))
//     }
// }
//
// impl Stream for Ps2Keyboard {
//     type Item = KeyPress;
//
//     fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<KeyPress>> {
//         let mut self_mut = self.get_mut();
//
//         if let Ok(value) = self_mut.read() {
//             if let Some(value) = value {
//                 return Poll::Ready(Some(value));
//             }
//         }
//
//         let mut waker = self_mut.get_waker();
//         waker.read().register(&cx.waker());
//
//         if let Ok(value) = self_mut.read() {
//             if let Some(value) = value {
//                 return Poll::Ready(Some(value));
//             }
//         }
//         Poll::Pending
//     }
// }
