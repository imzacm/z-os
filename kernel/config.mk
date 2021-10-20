# This file should not be modified, use config.mk in the project root for configuration

# Directory to build object files in
KERNEL_BUILD_DIR := $(BUILD_DIR)/kernel

KERNEL_SRC_DIR := $(KERNEL_ROOT_DIR)/src
KERNEL_ARCH_SRC_DIR := $(KERNEL_SRC_DIR)/arch/$(ARCH)
KERNEL_LINKER_SCRIPT := $(KERNEL_ARCH_SRC_DIR)/linker.ld

# Flags passed to $(CARGO)
KERNEL_CARGO_FLAGS := $(CARGO_FLAGS)
# Flags passed to $(ASM)
KERNEL_ASM_FLAGS := $(ASM_FLAGS)
# Flags passed to $(AS)
KERNEL_AS_FLAGS := $(AS_FLAGS)
# Flags passed to $(CC)
KERNEL_CC_FLAGS := $(CC_FLAGS)
# Flags passed to $(CXX)
KERNEL_CXX_FLAGS := $(CXX_FLAGS)
# Flags passed to $(LD)
KERNEL_LD_FLAGS := -T $(KERNEL_LINKER_SCRIPT) $(LD_FLAGS)
