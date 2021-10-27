use super::super::Port;

#[derive(Debug, Eq, PartialEq)]
pub struct Cursor<const WIDTH: usize, const HEIGHT: usize> {
    column: usize,
    row: usize,
    enabled: bool,
}

impl<const WIDTH: usize, const HEIGHT: usize> Cursor<WIDTH, HEIGHT> {
    const COMMAND_PORT: Port = Port::new(0x3D4);
    const DATA_PORT: Port = Port::new(0x3D5);

    pub const fn new() -> Self {
        Self {
            column: 0,
            row: 0,
            enabled: false,
        }
    }

    pub fn enable(&mut self) {
        if self.enabled {
            return;
        }
        self.enabled = true;
        unsafe {
            Self::COMMAND_PORT.write_u8(0x09);
            Self::DATA_PORT.write_u8(0x0F);
            Self::COMMAND_PORT.write_u8(0x0B);
            Self::DATA_PORT.write_u8(0x0F);
            Self::COMMAND_PORT.write_u8(0x0A);
            Self::DATA_PORT.write_u8(0x0E);
        }
    }

    #[allow(dead_code)]
    pub fn disable(&mut self) {
        if !self.enabled {
            return;
        }
        self.enabled = false;
        unsafe {
            Self::COMMAND_PORT.write_u8(0x0A);
            Self::DATA_PORT.write_u8(0x20);
        }
    }

    pub fn update(&mut self) {
        assert!(self.column < WIDTH && self.row < HEIGHT);
        if !self.enabled {
            return;
        }
        let position = (self.row * WIDTH + self.column) as u16;
        unsafe {
            Self::COMMAND_PORT.write_u8(0x0F);
            Self::DATA_PORT.write_u8((position & 0xFF) as u8);
            Self::COMMAND_PORT.write_u8(0x0E);
            Self::DATA_PORT.write_u8(((position >> 8) & 0xFF) as u8);
        }
    }

    pub fn point(&self) -> (usize, usize) {
        (self.column, self.row)
    }

    pub fn set_point(&mut self, column: usize, row: usize) {
        self.column = column;
        self.row = row;
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn set_column(&mut self, column: usize) {
        self.column = column
    }

    pub fn row(&self) -> usize {
        self.row
    }

    // pub fn set_row(&mut self, row: usize) {
    //     self.row = row;
    // }
}
