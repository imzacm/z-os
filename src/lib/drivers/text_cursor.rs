use spin::RwLock;
use x86_64::instructions::interrupts;
use x86_64::instructions::port::Port;
use crate::drivers::display::DisplayCursor;

#[derive(Debug)]
pub struct TextModeCursor {
    cursor: DisplayCursor,
    max_width: usize,
    max_height: usize,
    start: usize,
    end: usize,
}

impl TextModeCursor {
    pub fn new(max_width: usize, max_height: usize) -> TextModeCursor {
        let cursor = TextModeCursor {
            cursor: DisplayCursor { x: 0, y: 0 },
            max_width,
            max_height,
            start: 0,
            end: 15,
        };
        cursor.enable();
        cursor.update();

        cursor
    }

    pub fn enable(&self) {
        let mut low_port: Port<u8> = Port::new(0x3D4);
        let mut high_port: Port<u8> = Port::new(0x3D5);

        unsafe {
            let data: u8 = { high_port.read() };
            low_port.write(0x0A);
            high_port.write((data & 0xC0) | self.start as u8);
        }
        unsafe {
            let data: u8 = { high_port.read() };
            low_port.write(0x0B);
            high_port.write((data & 0xE0) | self.start as u8);
        }
    }

    pub fn disable(&self) {
        let mut low_port: Port<u8> = Port::new(0x3D4);
        let mut high_port: Port<u8> = Port::new(0x3D5);
        unsafe {
            low_port.write(0x0A);
            high_port.write(0x20);
        }
    }

    pub fn update(&self) {
        // Check position, then update if needed
        let virtual_cursor = self.get_cursor();
        let screen_cursor = self.get_screen_cursor();
        let delta = DisplayCursor {
            x: virtual_cursor.x - screen_cursor.x,
            y: virtual_cursor.y - screen_cursor.y,
        };
        crate::serial_println!("Virtual: {:?}, Physical: {:?}, Delta: {:?}", virtual_cursor, screen_cursor, delta);
    }

    pub fn get_screen_cursor(&self) -> DisplayCursor {
        let mut low_port: Port<u8> = Port::new(0x3D4);
        let mut high_port: Port<u8> = Port::new(0x3D5);
        let mut position: u16 = 0;
        unsafe {
            low_port.write(0x0F);
            position |= high_port.read() as u16;
            low_port.write(0x0E);
            position |= (high_port.read() as u16) << 8;
        }
        let y = position / (self.max_width as u16);
        let x = position % (self.max_width as u16);

        DisplayCursor { x: x as usize, y: y as usize }
    }

    pub fn get_cursor(&self) -> &DisplayCursor {
        &self.cursor
    }

    pub fn get_coordinates(&self) -> (usize, usize) {
        let cursor = &self.cursor;
        (cursor.x, cursor.y)
    }

    pub fn set_cursor(&mut self, cursor: DisplayCursor) {
        let mut current = &mut self.cursor;
        *current = cursor;
        self.update();
    }
}

//     fn _update_cursor(&self) {
//         use x86_64::instructions::port::Port;
//         let cursor = self.cursor.read();
//         let mut port1: Port<u8> = Port::new(0x3D4);
//         let mut port2: Port<u8> = Port::new(0x3D5);
//         let position: u16 = (cursor.y * VGA_WIDTH + cursor.x) as u16;
//         unsafe {
//             port1.write(0x0F);
//             port2.write((position & 0xFF) as u8);
//             port1.write(0x0E);
//             port2.write(((position >> 8) & 0xFF) as u8);
//         }
//     }
