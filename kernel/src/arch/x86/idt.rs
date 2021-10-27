use super::TablePointer;
use conquer_once::spin::OnceCell;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct DataSegmentRegisterState {
    /// GS: Extra segment register available for far pointer addressing
    pub gs: u32,
    /// FS: Extra segment register available for far pointer addressing
    pub fs: u32,
    /// ES: Extra segment register available for far pointer addressing
    pub es: u32,
    /// DS: Holds the data segment
    pub ds: u32,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct GeneralPurposeRegisterState {
    /// EDI: Destination index register, used for string, memory array copying and setting,
    /// and for far pointer addressing with ES
    pub edi: u32,
    /// ESI: Source index register, used for string and memory array copying
    pub esi: u32,
    /// EBP: Stack base pointer register, holds the base address of the stack
    pub ebp: u32,
    /// ESP: Stack pointer register, holds the top address of the stack
    pub esp: u32,
    /// EBX: Base register, used as a base pointer for memory access
    pub ebx: u32,
    /// EDX: // Data register, used for IO port access, arithmetic, some interrupt calls
    pub edx: u32,
    /// ECX: Counter register, used as a loop counter and for shifts
    pub ecx: u32,
    /// EAX: Accumulator register, used for IO port access, arithmetic, interrupt calls
    pub eax: u32,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct InterruptStackFrame {
    pub general_purpose: GeneralPurposeRegisterState,
    pub data_segment: DataSegmentRegisterState,
    /// ISRs push this as the interrupt index e.g. DivideByZero will be 0,
    /// IRQs push this as the IRQ index e.g. PIT timer will be 0.
    pub interrupt_num: u32,
    pub error_code: u32,
    pub eflags: u32,
    pub code_selector: u32,
    /// EIP
    pub instruction_pointer: u32,
}

#[allow(dead_code)]
#[repr(C, packed)]
#[derive(Copy, Clone)]
struct IdtEntry {
    base_low: u16,
    selector: u16,
    zero: u8,
    flags: u8,
    base_high: u16,
}

impl IdtEntry {
    const fn not_present() -> Self {
        Self {
            base_low: 0,
            selector: 0,
            zero: 0,
            flags: 0,
            base_high: 0,
        }
    }

    fn new(handler: unsafe extern "C" fn(), selector: u16, flags: u8) -> Self {
        let base = handler as usize as u64;
        Self {
            base_low: (base & 0xFFFF) as u16,
            selector,
            zero: 0,
            flags,
            base_high: ((base >> 16) & 0xFFFF) as u16,
        }
    }
}

const IDT_LEN: usize = 48;
static IDT: OnceCell<[IdtEntry; IDT_LEN]> = OnceCell::uninit();
static IDTR: OnceCell<TablePointer> = OnceCell::uninit();

pub unsafe fn setup_idt() {
    let idt = IDT.get_or_init(|| {
        let mut idt = [IdtEntry::not_present(); IDT_LEN];

        macro_rules! set_handler {
            ($index:expr, $ident:ident) => {
                {
                    extern "C" {
                        fn $ident();
                    }
                    idt[$index] = IdtEntry::new($ident, 0x08, 0x8E);
                }
            }
        }

        // Setup ISRs
        set_handler!(0, interrupt_handler_0);
        set_handler!(1, interrupt_handler_1);
        set_handler!(2, interrupt_handler_2);
        set_handler!(3, interrupt_handler_3);
        set_handler!(4, interrupt_handler_4);
        set_handler!(5, interrupt_handler_5);
        set_handler!(6, interrupt_handler_6);
        set_handler!(7, interrupt_handler_7);
        set_handler!(8, interrupt_handler_8);
        set_handler!(9, interrupt_handler_9);
        set_handler!(10, interrupt_handler_10);
        set_handler!(11, interrupt_handler_11);
        set_handler!(12, interrupt_handler_12);
        set_handler!(13, interrupt_handler_13);
        set_handler!(14, interrupt_handler_14);
        set_handler!(15, interrupt_handler_15);
        set_handler!(16, interrupt_handler_16);
        set_handler!(17, interrupt_handler_17);
        set_handler!(18, interrupt_handler_18);
        set_handler!(19, interrupt_handler_19);
        set_handler!(20, interrupt_handler_20);
        set_handler!(21, interrupt_handler_21);
        set_handler!(22, interrupt_handler_22);
        set_handler!(23, interrupt_handler_23);
        set_handler!(24, interrupt_handler_24);
        set_handler!(25, interrupt_handler_25);
        set_handler!(26, interrupt_handler_26);
        set_handler!(27, interrupt_handler_27);
        set_handler!(28, interrupt_handler_28);
        set_handler!(29, interrupt_handler_29);
        set_handler!(30, interrupt_handler_30);
        set_handler!(31, interrupt_handler_31);

        // Setup IRQs
        set_handler!(32, pic_interrupt_handler_0);
        set_handler!(33, pic_interrupt_handler_1);
        set_handler!(34, pic_interrupt_handler_2);
        set_handler!(35, pic_interrupt_handler_3);
        set_handler!(36, pic_interrupt_handler_4);
        set_handler!(37, pic_interrupt_handler_5);
        set_handler!(38, pic_interrupt_handler_6);
        set_handler!(39, pic_interrupt_handler_7);
        set_handler!(40, pic_interrupt_handler_8);
        set_handler!(41, pic_interrupt_handler_9);
        set_handler!(42, pic_interrupt_handler_10);
        set_handler!(43, pic_interrupt_handler_11);
        set_handler!(44, pic_interrupt_handler_12);
        set_handler!(45, pic_interrupt_handler_13);
        set_handler!(46, pic_interrupt_handler_14);
        set_handler!(47, pic_interrupt_handler_15);

        idt
    });

    let idtr = IDTR.get_or_init(|| TablePointer::new::<IdtEntry, IDT_LEN>(idt));
    asm!("lidt [{}]", in(reg) idtr, options(readonly, nostack, preserves_flags));
}

#[no_mangle]
extern "C" fn x86_interrupt_handler(stack_frame: InterruptStackFrame) {
    use crate::engine::{Exception, handle_exception};

    match stack_frame.interrupt_num {
        0 => panic!("Divide by zero: {:?}", stack_frame),
        4 => handle_exception(Exception::Overflow, stack_frame.instruction_pointer),
        5 => handle_exception(Exception::BoundRangeExceeded, stack_frame.instruction_pointer),
        6 => handle_exception(Exception::InvalidInstruction, stack_frame.instruction_pointer),
        7 => handle_exception(Exception::DeviceNotAvailable, stack_frame.instruction_pointer),
        8 => panic!("Double fault: {:?}", stack_frame),
        10 | 11 | 12 | 13 | 14 => handle_exception(Exception::MemoryFault, stack_frame.instruction_pointer),
        20 => handle_exception(Exception::Virtualization, stack_frame.instruction_pointer),
        30 => handle_exception(Exception::Security, stack_frame.instruction_pointer),
        _ => return
    }
}
