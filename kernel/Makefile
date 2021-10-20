KERNEL_ROOT_DIR := $(ROOT_DIR)/kernel

include $(KERNEL_ROOT_DIR)/config.mk

KERNEL_BIN := $(KERNEL_BUILD_DIR)/kernel-$(ARCH).bin
KERNEL_DIST_BIN := $(DIST_DIR)/iso/boot/kernel.bin
KERNEL_RUST_OBJ := $(KERNEL_BUILD_DIR)/kernel-rust.a

KERNEL_ARCH_SRCS := $(wildcard $(KERNEL_ARCH_SRC_DIR)/*.S)
KERNEL_ARCH_SRCS += $(wildcard $(KERNEL_ARCH_SRC_DIR)/*.asm)
KERNEL_ARCH_SRCS += $(wildcard $(KERNEL_ARCH_SRC_DIR)/*.c)
KERNEL_ARCH_SRCS += $(wildcard $(KERNEL_ARCH_SRC_DIR)/*.cpp)
KERNEL_ARCH_OBJS := $(patsubst $(KERNEL_ARCH_SRC_DIR)/%, $(KERNEL_BUILD_DIR)/arch/%.o, $(KERNEL_ARCH_SRCS))

KERNEL_OBJS := $(KERNEL_BUILD_DIR)/arch/crtbegin.o $(KERNEL_ARCH_OBJS) $(KERNEL_RUST_OBJ) $(KERNEL_BUILD_DIR)/arch/crtend.o

.PHONY: kernel kernel-rust kernel-arch

kernel: $(KERNEL_DIST_BIN)

kernel-rust: $(KERNEL_RUST_OBJ)

kernel-arch: $(KERNEL_ARCH_OBJS)

$(KERNEL_DIST_BIN): $(DIST_DIR) $(KERNEL_BIN)
	@cp $(KERNEL_BIN) $@

$(KERNEL_BIN): $(KERNEL_OBJS)
	@$(LD) $(KERNEL_LD_FLAGS) -n -o $@ $^

$(KERNEL_RUST_OBJ):
	@mkdir -p $(shell dirname $@)
	@$(CARGO) build $(KERNEL_CARGO_FLAGS) --package kernel
	@cp $(ROOT_DIR)/target/$(RUST_TARGET_NAME)/$(PROFILE)/libkernel.a $@

$(KERNEL_BUILD_DIR)/arch/crtbegin.o $(KERNEL_BUILD_DIR)/arch/crtend.o:
	@mkdir -p $(shell dirname $@)
	@OBJ=`$(CC) $(KERNEL_CC_FLAGS) -print-file-name=$(@F)` && cp "$$OBJ" $@

$(KERNEL_BUILD_DIR)/arch/%.asm.o: $(KERNEL_ARCH_SRC_DIR)/%.asm
	@mkdir -p $(shell dirname $@)
	@$(ASM) $(KERNEL_ASM_FLAGS) -o $@ $<

$(KERNEL_BUILD_DIR)/arch/%.S.o: $(KERNEL_ARCH_SRC_DIR)/%.S
	@mkdir -p $(shell dirname $@)
	@$(AS) $(KERNEL_AS_FLAGS) -o $@ $<

$(KERNEL_BUILD_DIR)/arch/%.c.o: $(KERNEL_ARCH_SRC_DIR)/%.c
	@mkdir -p $(shell dirname $@)
	@$(CC) $(KERNEL_CC_FLAGS) -o $@ $<

$(KERNEL_BUILD_DIR)/arch/%.cpp.o: $(KERNEL_ARCH_SRC_DIR)/%.cpp
	@mkdir -p $(shell dirname $@)
	@$(CXX) $(KERNEL_CXX_FLAGS) -o $@ $<
