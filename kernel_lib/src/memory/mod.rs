use bootloader::BootInfo;
use x86_64::VirtAddr;
use x86_64::structures::paging::mapper::MapToError;
use x86_64::structures::paging::Size4KiB;

pub mod gdt;
pub mod paging;
pub mod alloc;

#[derive(Debug)]
pub enum MemoryInitError {
    DuringAllocation(MapToError<Size4KiB>)
}

pub fn memory_init(boot_info: &'static BootInfo, init_alloc: bool) -> Result<(), MemoryInitError> {
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { paging::init(phys_mem_offset) };
    let mut frame_allocator = unsafe {
        paging::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    if init_alloc {
        alloc::init(&mut mapper, &mut frame_allocator)
            .map_err(|error| MemoryInitError::DuringAllocation(error))?;
    }
    Ok(())
}
