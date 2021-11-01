use super::{Status, ControllerConfig, Ps2DeviceType};
use crate::arch::{Port, io_wait};

const DATA_PORT: Port = Port::new(0x60);
const COMMAND_PORT: Port = Port::new(0x64);

const READ_TIMEOUT: usize = 5;
const WRITE_TIMEOUT: usize = 5;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Ps2IoError {
    Timeout,
    UnexpectedResponse(u8),
}

pub unsafe fn read_byte_unchecked() -> u8 {
    DATA_PORT.read_u8()
}

fn read_byte() -> Result<u8, Ps2IoError> {
    let mut count = 0;
    loop {
        if !read_status().output_buffer_empty() {
            return Ok(unsafe { DATA_PORT.read_u8() });
        }
        if count == READ_TIMEOUT {
            return Err(Ps2IoError::Timeout);
        }
        count += 1;
        unsafe { io_wait() };
    }
}

fn read(buffer: &mut [u8]) -> Result<usize, Ps2IoError> {
    let len = buffer.len();
    let mut index = 0;
    loop {
        match read_byte() {
            Ok(byte) => { buffer[index] = byte; }
            Err(Ps2IoError::Timeout) => return Ok(index + 1),
            Err(error) => return Err(error)
        }
        if index == len - 1 {
            return Ok(index + 1);
        }
        index += 1;
    }
}

fn write(port: Port, data: u8) -> Result<(), Ps2IoError> {
    let mut count = 0;
    loop {
        if read_status().input_buffer_empty() {
            unsafe { port.write_u8(data) };
            return Ok(());
        }
        if count == WRITE_TIMEOUT {
            return Err(Ps2IoError::Timeout);
        }
        count += 1;
        unsafe { io_wait() };
    }
}

fn write_command(command: u8) -> Result<(), Ps2IoError> {
    write(COMMAND_PORT, command)
}

fn write_data(data: u8) -> Result<(), Ps2IoError> {
    write(DATA_PORT, data)
}

pub fn flush_output_buffer() {
    while !read_status().output_buffer_empty() {
        unsafe { DATA_PORT.read_u8() };
    }
}

pub fn read_status() -> Status {
    let value = unsafe { COMMAND_PORT.read_u8() };
    Status::new(value)
}

pub fn read_controller_config() -> Result<ControllerConfig, Ps2IoError> {
    write(COMMAND_PORT, 0x20)?;
    let value = read_byte()?;
    Ok(ControllerConfig::new(value))
}

pub fn write_controller_config(config: ControllerConfig) -> Result<(), Ps2IoError> {
    write(COMMAND_PORT, 0x60)?;
    write(DATA_PORT, config.value())?;
    Ok(())
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Ps2ControllerTestError {
    Io(Ps2IoError),
    Failed,
}

impl From<Ps2IoError> for Ps2ControllerTestError {
    fn from(error: Ps2IoError) -> Self {
        Self::Io(error)
    }
}

pub fn test_controller() -> Result<(), Ps2ControllerTestError> {
    write_command(0xAA)?;
    match read_byte()? {
        0x55 => Ok(()),
        0xFC => Err(Ps2ControllerTestError::Failed),
        byte => Err(Ps2ControllerTestError::Io(Ps2IoError::UnexpectedResponse(byte)))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Ps2PortTestError {
    Io(Ps2IoError),
    ClockLineStuckLow,
    ClockLineStuckHigh,
    DataLineStuckLow,
    DataLineStuckHigh,
}

impl From<Ps2IoError> for Ps2PortTestError {
    fn from(error: Ps2IoError) -> Self {
        Self::Io(error)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Ps2Port {
    First,
    Second,
}

pub fn disable_port(port: Ps2Port) -> Result<(), Ps2IoError> {
    match port {
        Ps2Port::First => write_command(0xAD),
        Ps2Port::Second => write_command(0xA7),
    }
}

pub fn enable_port(port: Ps2Port) -> Result<(), Ps2IoError> {
    match port {
        Ps2Port::First => write_command(0xAE),
        Ps2Port::Second => write_command(0xA8),
    }
}

pub fn test_port(port: Ps2Port) -> Result<(), Ps2PortTestError> {
    match port {
        Ps2Port::First => write_command(0xAB)?,
        Ps2Port::Second => write_command(0xA9)?,
    }
    match read_byte()? {
        0x00 => Ok(()),
        0x01 => Err(Ps2PortTestError::ClockLineStuckLow),
        0x02 => Err(Ps2PortTestError::ClockLineStuckHigh),
        0x03 => Err(Ps2PortTestError::DataLineStuckLow),
        0x04 => Err(Ps2PortTestError::DataLineStuckHigh),
        byte => Err(Ps2PortTestError::Io(Ps2IoError::UnexpectedResponse(byte)))
    }
}

/// Returns `true` if the device responded with 0xFA (success)
pub fn reset_device(port: Ps2Port) -> Result<bool, Ps2IoError> {
    if port == Ps2Port::Second {
        write_command(0xD4)?;
    }
    write_data(0xFF)?;
    if let Ok(0xFA) = read_byte() {
        if let Ok(0xAA) = read_byte() {
            return Ok(true);
        }
    }
    Ok(false)
}

pub fn disable_scanning(port: Ps2Port) -> Result<(), Ps2IoError> {
    if port == Ps2Port::Second {
        write_command(0xD4)?;
    }
    write_command(0xF5)?;
    let b = read_byte();
    crate::kprintln!("Disable scanning response: {:?}", b);
    match b? {
        0xFA => Ok(()),
        byte => Err(Ps2IoError::UnexpectedResponse(byte))
    }
}

pub fn identify_device(port: Ps2Port) -> Result<Ps2DeviceType, Ps2IoError> {
    if port == Ps2Port::Second {
        write_command(0xD4)?;
    }
    write_command(0xF2)?;
    match read_byte()? {
        0xFA => {}
        byte => return Err(Ps2IoError::UnexpectedResponse(byte))
    }
    let mut buffer = [0; 2];
    let len = read(&mut buffer)?;
    match (len, buffer[0], buffer[1]) {
        (0, _, _) => Ok(Ps2DeviceType::AtKeyboard),
        (1, 0x00, _) => Ok(Ps2DeviceType::StandardMouse),
        (1, 0x03, _) => Ok(Ps2DeviceType::ScrollMouse),
        (1, 0x04, _) => Ok(Ps2DeviceType::FiveBtnMouse),
        (2, 0xAB, 0x41) | (2, 0xAB, 0xC1) => Ok(Ps2DeviceType::Mf2Keyboard),
        (_, byte, _) => Err(Ps2IoError::UnexpectedResponse(byte))
    }
}
