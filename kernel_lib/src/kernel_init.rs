use bootloader::BootInfo;
use crate::memory::{MemoryInitError, memory_init};
use bootloader::bootinfo::MemoryRegionType::Kernel;

#[derive(Debug)]
pub enum KernelInitError {
    Memory(MemoryInitError)
}

#[derive(Debug, Copy, Clone)]
pub struct KernelInit {
    init_gdt: bool,
    init_interrupts: bool,
    init_memory: Option<&'static BootInfo>,
    init_alloc: bool,
}

impl KernelInit {
    pub fn none() -> KernelInit {
        KernelInit {
            init_gdt: false,
            init_interrupts: false,
            init_memory: None,
            init_alloc: false,
        }
    }

    pub fn default(boot_info: Option<&'static BootInfo>) -> KernelInit {
        KernelInit {
            init_gdt: true,
            init_interrupts: true,
            init_memory: boot_info,
            init_alloc: true,
        }
    }

    pub fn apply(self) -> Result<(), KernelInitError> {
        if self.init_gdt {
            crate::memory::gdt::init();
        }
        if self.init_interrupts {}
        if let Some(boot_info) = self.init_memory {
            memory_init(boot_info, self.init_alloc)
                .map_err(|error| KernelInitError::Memory(error))?;
        }
        Ok(())
    }

    pub fn enable_gdt(&mut self) -> &mut Self {
        self.init_gdt = true;
        self
    }

    pub fn disable_gdt(&mut self) -> &mut Self {
        self.init_gdt = false;
        self
    }

    pub fn enable_memory(&mut self, boot_info: &'static BootInfo) -> &mut Self {
        self.init_memory = Some(boot_info);
        self
    }

    pub fn disable_memory(&mut self) -> &mut Self {
        self.init_memory = None;
        self.init_alloc = false;
        self
    }

    pub fn memory_with_alloc(&mut self, boot_info: &'static BootInfo) -> &mut Self {
        self.init_memory = Some(boot_info);
        self.init_alloc = true;
        self
    }
}
