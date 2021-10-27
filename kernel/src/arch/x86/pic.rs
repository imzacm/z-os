use super::{Port, io_wait};
use super::InterruptStackFrame;

#[derive(Debug, Eq, PartialEq)]
struct Pic {
    command_port: Port,
    data_port: Port,
    offset: u8,
}

impl Pic {
    const fn new(command_port: u16, offset: u8) -> Self {
        Self {
            command_port: Port::new(command_port),
            data_port: Port::new(command_port + 1),
            offset,
        }
    }

    unsafe fn write_command(&self, command: u8) {
        self.command_port.write_u8(command);
    }

    unsafe fn write_data(&self, data: u8) {
        self.data_port.write_u8(data);
    }

    unsafe fn read_data(&self) -> u8 {
        self.data_port.read_u8()
    }

    unsafe fn send_eoi(&self) {
        self.write_command(0x20);
    }
}

/// Based on the code at https://wiki.osdev.org/8259_PIC
unsafe fn remap_pics(pic_1: &Pic, pic_2: &Pic) {
    // ICW4 (not) needed
    const ICW1_ICW4: u8 = 0x01;
    // Initialization - required!
    const ICW1_INIT: u8 = 0x10;

    // 8086/88 (MCS-80/85) mode
    const ICW4_8086: u8 = 0x01;

    // Save masks
    let pic1_mask = pic_1.read_data();
    let pic2_mask = pic_2.read_data();

    // Start init
    pic_1.write_command(ICW1_INIT | ICW1_ICW4);
    io_wait();
    pic_2.write_command(ICW1_INIT | ICW1_ICW4);
    io_wait();
    // ICW2: Master PIC vector offset
    pic_1.write_data(pic_1.offset);
    io_wait();
    // ICW2: Slave PIC vector offset
    pic_2.write_data(pic_2.offset);
    io_wait();
    // ICW3: tell Master PIC that there is a slave PIC at IRQ2 (0000 0100)
    pic_1.write_data(4);
    io_wait();
    // ICW3: tell Slave PIC its cascade identity (0000 0010)
    pic_2.write_data(2);
    io_wait();
    pic_1.write_data(ICW4_8086);
    io_wait();
    pic_2.write_data(ICW4_8086);
    io_wait();

    // Restore masks
    pic_1.write_data(pic1_mask);
    pic_2.write_data(pic2_mask);
}

const PIC_1: Pic = Pic::new(0x20, 0x20);
const PIC_2: Pic = Pic::new(0xA0, 0x28);

pub unsafe fn setup_pic() {
    remap_pics(&PIC_1, &PIC_2);
}

#[no_mangle]
extern "C" fn x86_pic_interrupt_handler(stack_frame: InterruptStackFrame) {
    use crate::engine::{Interrupt, handle_interrupt};

    let interrupt = match stack_frame.interrupt_num {
        0 => Some(Interrupt::Timer),
        _ => None
    };
    if let Some(interrupt) = interrupt {
        handle_interrupt(interrupt);
    }

    unsafe {
        if stack_frame.interrupt_num >= 8 {
            PIC_2.send_eoi();
        }
        PIC_1.send_eoi();
    }
}
