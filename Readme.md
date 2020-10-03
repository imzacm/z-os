# Z-OS Attempt 7

## Structure
The project is split into multiple crates to keep everything separated.

### Kernel
The kernel crate is the entry point to the kernel, it handles setup, panics, etc.

### Kernel lib
The kernel lib crate is the "behind the scenes" part of the project. It handles any direct communication with any hardware.

### Driver lib
The driver lib crate exposes a driver interface to the kernel. The reason this is in a separate crate is to clearly separate between interrupts, etc and the actual "user" facing interface.

### PC keyboard
PC keyboard is pulled in as a submodule and is included in the workspace. The only reason it's a part of the project is to expose ```keyboard.modifiers``` and add the ```Debug``` trait.
