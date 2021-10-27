use super::cursor::Cursor;

#[allow(dead_code)]
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
struct Buffer<const WIDTH: usize, const HEIGHT: usize> {
    rows: [[ScreenChar; WIDTH]; HEIGHT],
}

pub struct Screen<const WIDTH: usize, const HEIGHT: usize> {
    cursor: Cursor<WIDTH, HEIGHT>,
    colour: ColourCode,
    buffer: &'static mut Buffer<WIDTH, HEIGHT>,
}

impl<const WIDTH: usize, const HEIGHT: usize> Screen<WIDTH, HEIGHT> {
    pub fn new() -> Self {
        let mut cursor = Cursor::new();
        cursor.enable();
        let colour = ColourCode::new(Colour::White, Colour::Black);
        let buffer = unsafe {
            let blank = ScreenChar {
                character: b' ',
                colour,
            };
            let buffer = &mut *(0xb8000 as *mut Buffer<WIDTH, HEIGHT>);
            for row in 0..HEIGHT {
                for column in 0..WIDTH {
                    core::ptr::write_volatile(&mut buffer.rows[row][column], blank);
                }
            }
            buffer
        };

        Self { cursor, colour, buffer }
    }

    fn new_line(&mut self, update_cursor: bool) {
        let mut row = self.cursor.row() + 1;
        if row >= HEIGHT {
            for row in 0..HEIGHT - 1 {
                for column in 0..WIDTH {
                    let char = self.buffer.rows[row + 1][column];
                    unsafe { core::ptr::write_volatile(&mut self.buffer.rows[row][column], char) };
                }
            }
            row -= 1;
            let blank_char = ScreenChar {
                character: b' ',
                colour: self.colour,
            };
            for column in 0..WIDTH {
                unsafe { core::ptr::write_volatile(&mut self.buffer.rows[row][column], blank_char) };
            }
        }
        self.cursor.set_point(0, row);
        if update_cursor {
            self.cursor.update();
        }
    }

    fn write_byte(&mut self, byte: u8, update_cursor: bool) {
        if byte == b'\n' {
            self.new_line(true);
            return;
        }
        let (column, row) = self.cursor.point();
        let char = ScreenChar {
            character: byte,
            colour: self.colour,
        };
        unsafe { core::ptr::write_volatile(&mut self.buffer.rows[row][column], char) };
        self.cursor.set_column(column + 1);
        if self.cursor.column() >= WIDTH {
            self.new_line(update_cursor);
        } else if update_cursor {
            self.cursor.update();
        }
    }

    pub fn write_str(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // ASCII or new line
                0x20..0x7e | b'\n' => self.write_byte(byte, false),
                _ => self.write_byte(0xfe, false)
            }
        }
        self.cursor.update();
    }
}
