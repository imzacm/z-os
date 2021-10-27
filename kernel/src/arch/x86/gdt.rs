use super::TablePointer;
use conquer_once::spin::OnceCell;

#[repr(C, packed)]
#[derive(Copy, Clone)]
struct GdtEntry {
    limit_low: u16,
    base_low: u16,
    base_middle: u8,
    access: u8,
    granularity: u8,
    base_high: u8,
}

impl GdtEntry {
    const fn new(base: u64, limit: u64, access: u8, granularity: u8) -> Self {
        let base_low = (base & 0xFFFF) as u16;
        let base_middle = ((base >> 16) & 0xFF) as u8;
        let base_high = ((base >> 24) & 0xFF) as u8;
        let limit_low = (limit & 0xFFFF) as u16;
        let mut gran = ((limit >> 16) & 0x0F) as u8;
        gran |= granularity & 0xF0;
        Self {
            limit_low,
            base_low,
            base_middle,
            access,
            granularity: gran,
            base_high,
        }
    }
}

const GDT_LEN: usize = 3;
static GDT: [GdtEntry; GDT_LEN] = [
    GdtEntry::new(0, 0, 0, 0),
    GdtEntry::new(0, 0xFFFFFFFF, 0x9A, 0xCF),
    GdtEntry::new(0, 0xFFFFFFFF, 0x92, 0xCF),
];
static GDTR: OnceCell<TablePointer> = OnceCell::uninit();

pub unsafe fn setup_gdt() {
    extern "C" {
        fn reload_segments();
    }

    let gdtr = GDTR.get_or_init(|| TablePointer::new::<GdtEntry, GDT_LEN>(&GDT));
    asm!("lgdt [{}]", in(reg) gdtr, options(readonly, nostack, preserves_flags));
    reload_segments();
}
