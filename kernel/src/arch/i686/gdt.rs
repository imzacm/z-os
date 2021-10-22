#[repr(packed)]
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
    const fn default() -> Self {
        Self {
            limit_low: 0,
            base_low: 0,
            base_middle: 0,
            access: 0,
            granularity: 0,
            base_high: 0,
        }
    }
}

#[repr(packed)]
struct GdtDescriptor {
    limit: u16,
    base: u32,
}

impl GdtDescriptor {
    const fn default() -> Self {
        Self { limit: 0, base: 0 }
    }
}

static mut GDT: [GdtEntry; 3] = [GdtEntry::default(); 3];
#[no_mangle]
static mut GDTR: GdtDescriptor = GdtDescriptor::default();

unsafe fn set_gdt_entry(index: usize, base: u64, limit: u64, access: u8, granularity: u8) {
    assert!(index < 3);
    let entry = &mut GDT[index];
    entry.base_low = (base & 0xFFFF) as u16;
    entry.base_middle = ((base >> 16) & 0xFF) as u8;
    entry.base_high = ((base >> 24) & 0xFF) as u8;
    entry.limit_low = (limit & 0xFFFF) as u16;
    entry.granularity = ((limit >> 16) & 0x0F) as u8;
    entry.granularity |= granularity & 0xF0;
    entry.access = access;
}

extern "C" {
    fn load_gdt_descriptor();
}

pub unsafe fn init_gdt() {
    set_gdt_entry(0, 0, 0, 0, 0);
    set_gdt_entry(1, 0, 0xFFFFFFFF, 0x9A, 0xCF);
    set_gdt_entry(2, 0, 0xFFFFFFFF, 0x92, 0xCF);

    GDTR.limit = ((core::mem::size_of::<GdtEntry>() * 3) - 1) as u16;
    GDTR.base = &GDT as *const [GdtEntry; 3] as usize as u32;
    load_gdt_descriptor();
}
