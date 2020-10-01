use crate::result::{OsResult, GenericOsError};
use spin::{RwLock, Mutex};
use volatile::Volatile;
use dia_semver::Semver;
use alloc::string::String;
use core::fmt;
use x86_64::instructions::port::Port;
use x86_64::instructions::interrupts;
use crate::drivers::text_cursor::TextModeCursor;
use crate::drivers::display::DisplayCursor;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
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
struct ColorCode(u8);

impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

pub const VGA_HEIGHT: usize = 25;
pub const VGA_WIDTH: usize = 80;

#[derive(Debug, Clone)]
#[repr(transparent)]
struct VgaDisplayBuffer {
    chars: [[Volatile<ScreenChar>; VGA_WIDTH]; VGA_HEIGHT]
}

#[derive(Debug)]
pub struct VgaDisplayDriver<'a> {
    cursor: &'a mut TextModeCursor,
    color_code: ColorCode,
    buffer: &'static mut VgaDisplayBuffer,
}

impl<'a> VgaDisplayDriver<'a> {
    pub fn new(fg: Color, bg: Color, cursor: &'a mut TextModeCursor) -> VgaDisplayDriver<'a> {
        VgaDisplayDriver {
            cursor,
            color_code: ColorCode::new(fg, bg),
            buffer: unsafe { &mut *(0xb8000 as *mut VgaDisplayBuffer) },
        }
    }

    fn _clear_row(&mut self, row: usize) -> OsResult<()> {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..VGA_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
        Ok(())
    }

    fn _move_up(&mut self, n: usize) -> OsResult<()> {
        let blank = ScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        let (_, y) = self.cursor.get_coordinates();
        for row in 0..y {
            for col in 0..VGA_WIDTH {
                let to_move = self.buffer.chars[row + 1][col].read();
                self.buffer.chars[row][col].write(to_move);
                self.buffer.chars[row + 1][col].write(blank);
            }
        }
        Ok(())
    }

    pub fn create_restricted(&mut self, x: usize, y: usize, width: usize, height: usize) -> OsResult<()> {

    }

    fn print_string(&mut self, str: &str) -> OsResult<()> {
        let (x, y) = self.cursor.get_coordinates();
        let (x, y) = self.print_string_at((x, y), str)?;
        self.cursor.set_cursor(DisplayCursor { x, y });
        Ok(())
    }

    fn print_string_at(&mut self, coords: (usize, usize), str: &str) -> OsResult<(usize, usize)> {
        let mut coords = coords;
        for byte in str.bytes() {
            coords = match byte {
                0x20..=0x7e | b'\n' => self.print_byte_at(coords, &byte)?,
                _ => self.print_byte_at(coords, &0xfe)?
            }
        }
        Ok(coords)
    }

    fn print_byte(&mut self, byte: &u8) -> OsResult<()> {
        let coords = self.cursor.get_coordinates();
        let (x, y) = self.print_byte_at(coords, byte)?;
        self.cursor.set_cursor(DisplayCursor { x, y });
        Ok(())
    }

    fn print_byte_at(&mut self, (x, y): (usize, usize), byte: &u8) -> OsResult<(usize, usize)> {
        let mut x = x;
        let mut y = y;
        match byte {
            b'\n' => {
                y += 1;
                x = 0;
            }
            byte => {
                let color_code = self.color_code;
                self.buffer.chars[y][x].write(ScreenChar {
                    ascii_character: byte.clone(),
                    color_code,
                });
                x += 1;
            }
        }
        if x == VGA_WIDTH {
            y += 1;
            x = 0;
        }
        if y == VGA_HEIGHT {
            y -= 1;
            self._move_up(1);
        }
        Ok((x, y))
    }
}

impl<'a> fmt::Write for VgaDisplayDriver<'a> {
    fn write_str(&mut self, str: &str) -> fmt::Result {
        self.print_string(str);
        Ok(())
    }
}
