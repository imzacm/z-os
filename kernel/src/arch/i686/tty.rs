use super::io::{outb, inb};

const HEIGHT: usize = 25;
const WIDTH: usize = 80;

struct Cursor {
    column: usize,
    row: usize,
    enabled: bool,
}

impl Cursor {
    const COMMAND_PORT: u16 = 0x3D4;
    const DATA_PORT: u16 = 0x3D5;

    const fn default() -> Self {
        Self { column: 0, row: 0, enabled: false }
    }

    fn enable(&mut self) {
        if self.enabled {
            return;
        }
        self.enabled = true;
        // unsafe {
        //     outb(Self::COMMAND_PORT, 0x0A);
        //     outb(Self::DATA_PORT, (inb(Self::DATA_PORT) | ));
        // }
    }

    fn disable(&mut self) {
        if !self.enabled {
            return;
        }
        self.enabled = false;
    }

    /// Update the screen cursor with self.column and self.row
    fn update(&mut self) {
        assert!(self.column < WIDTH && self.row < HEIGHT);
        if !self.enabled {
            return;
        }
        let position = (self.column * 80 + self.row) as u16;
        unsafe {
            outb(Self::COMMAND_PORT, 14);
            outb(Self::DATA_PORT, (position >> 8) as u8);
            outb(Self::COMMAND_PORT, 15);
            outb(Self::DATA_PORT, position as u8);
        }
    }

    fn point(&self) -> (usize, usize) {
        (self.column, self.row)
    }

    fn set_point(&mut self, column: usize, row: usize) {
        self.column = column;
        self.row = row;
    }

    fn column(&self) -> usize {
        self.column
    }

    fn set_column(&mut self, column: usize) {
        self.column = column
    }

    fn row(&self) -> usize {
        self.row
    }

    fn set_row(&mut self, row: usize) {
        self.row = row;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Colour {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColourCode(u8);

impl ColourCode {
    const fn new(foreground: Colour, background: Colour) -> Self {
        Self((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    character: u8,
    colour: ColourCode,
}

#[repr(transparent)]
struct Buffer {
    rows: [[ScreenChar; WIDTH]; HEIGHT],
}

struct Tty {
    cursor: Cursor,
    colour: ColourCode,
    buffer: &'static mut Buffer,
}

impl Tty {
    fn new() -> &'static mut Self {
        static mut TTY: Option<Tty> = None;
        unsafe {
            if let Some(tty) = TTY.as_mut() {
                return tty;
            }
        }
        let mut cursor = Cursor::default();
        cursor.enable();
        unsafe {
            TTY = Some(Self {
                cursor,
                colour: ColourCode::new(Colour::White, Colour::Black),
                buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
            });
            return TTY.as_mut().unwrap();
        }
    }

    fn new_line(&mut self, update_cursor: bool) {
        let row = self.cursor.row() + 1;
        if row >= HEIGHT {
            todo!("Scroll");
        }
        self.cursor.set_point(0, row);
        if update_cursor {
            self.cursor.update();
        }
    }

    fn write_byte(&mut self, byte: u8) {
        if byte == b'\n' {
            self.new_line(true);
            return;
        }
        let (column, row) = self.cursor.point();
        unsafe {
            core::ptr::write_volatile(&mut self.buffer.rows[row][column], ScreenChar {
                character: byte,
                colour: self.colour,
            });
        }
        self.cursor.set_column(column + 1);
        if self.cursor.column >= WIDTH {
            self.new_line(false);
        }
        self.cursor.update();
    }

    fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // ASCII or new line
                0x20..0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe)
            }
        }
    }
}

pub fn init_tty() {
    Tty::new();
}

pub fn print(s: &str) {
    Tty::new().write_str(s);
}

pub fn println(s: &str) {
    let tty = Tty::new();
    tty.write_str(s);
    tty.write_byte(b'\n');
}
