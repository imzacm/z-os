KERNEL_ROOT_DIR := $(realpath $(dir $(abspath $(lastword $(MAKEFILE_LIST)))))

KERNEL_BUILD_DIR := $(BUILD_DIR)/kernel
KERNEL_ARCH_SRC_DIR := $(KERNEL_ROOT_DIR)/src/arch/$(ARCH)
KERNEL_LINKER_SCRIPT := $(KERNEL_ROOT_DIR)/targets/$(ARCH)-linker.ld

# Flags passed to $(CARGO)
KERNEL_CARGO_FLAGS := $(CARGO_FLAGS) --package kernel
# Flags passed to $(ASM)
KERNEL_ASM_FLAGS := $(ASM_FLAGS)
# Flags passed to $(CC)
KERNEL_CC_FLAGS := $(CC_FLAGS)
# Flags passed to $(LD)
KERNEL_LD_FLAGS := $(LD_FLAGS) -n -T $(KERNEL_LINKER_SCRIPT)

KERNEL_BIN := $(KERNEL_BUILD_DIR)/kernel-$(ARCH).bin
KERNEL_DIST_BIN := $(DIST_DIR)/iso/boot/kernel.bin
KERNEL_RUST_OBJ := $(KERNEL_BUILD_DIR)/kernel-rust.a

KERNEL_ARCH_SRCS := \
	$(wildcard $(KERNEL_ARCH_SRC_DIR)/*.asm) $(wildcard $(KERNEL_ARCH_SRC_DIR)/**/*.asm) \
	$(wildcard $(KERNEL_ARCH_SRC_DIR)/*.c) $(wildcard $(KERNEL_ARCH_SRC_DIR)/**/*.c)
KERNEL_ARCH_OBJS := $(patsubst $(KERNEL_ARCH_SRC_DIR)/%, $(KERNEL_BUILD_DIR)/arch/%.o, $(KERNEL_ARCH_SRCS))

KERNEL_OBJS := $(KERNEL_ARCH_OBJS) $(KERNEL_RUST_OBJ)

.PHONY: kernel

kernel: $(KERNEL_DIST_BIN)

$(KERNEL_BUILD_DIR):
	@mkdir -p $@

$(KERNEL_DIST_BIN): $(DIST_DIR) $(KERNEL_BIN)
	@cp $(KERNEL_BIN) $@

$(KERNEL_BIN): $(KERNEL_OBJS)
	@$(LD) $(KERNEL_LD_FLAGS) -o $@ $^

$(KERNEL_RUST_OBJ): $(KERNEL_BUILD_DIR)
	@$(CARGO) build $(KERNEL_CARGO_FLAGS)
	@cp $(ROOT_DIR)/target/$(RUST_TARGET_NAME)/$(PROFILE)/libkernel.a $@

$(KERNEL_BUILD_DIR)/arch/%.asm.o: $(KERNEL_ARCH_SRC_DIR)/%.asm
	@mkdir -p $(shell dirname $@)
	@$(ASM) $(KERNEL_ASM_FLAGS) -o $@ $<

$(KERNEL_BUILD_DIR)/arch/%.c.o: $(KERNEL_ARCH_SRC_DIR)/%.c
	@mkdir -p $(shell dirname $@)
	@$(CC) $(KERNEL_CC_FLAGS) -o $@ $<
