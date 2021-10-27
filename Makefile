ROOT_DIR := $(realpath $(dir $(abspath $(lastword $(MAKEFILE_LIST)))))

include ./config.mk
include ./targets/config.mk

ifeq (,$(wildcard ./$(PREFIX)))
	PREFIX := $(ROOT_DIR)/$(PREFIX)
endif

BUILD_DIR := $(ROOT_DIR)/$(BUILD_DIR)
DIST_DIR := $(ROOT_DIR)/$(DIST_DIR)
DIST_ISO := $(DIST_DIR)/z_os.iso

CARGO := cargo
CC := $(PREFIX)/bin/$(TARGET_CPU)-$(TARGET_FORMAT)-gcc
LD := $(PREFIX)/bin/$(TARGET_CPU)-$(TARGET_FORMAT)-gcc
ASM := $(PREFIX)/bin/nasm

CARGO_FLAGS := --manifest-path $(ROOT_DIR)/Cargo.toml --target $(RUST_TARGET)
ifeq ($(PROFILE), release)
	CARGO_FLAGS += --release
endif
CC_FLAGS := -O2 -g -ffreestanding -Wall -Wextra -nostdlib -m32 -c
LD_FLAGS := -O2 -g -ffreestanding -Wall -Wextra -nostdlib -m32
ASM_FLAGS := -f elf

.PHONY: all clean kernel iso run debug

all: iso

clean:
	@rm -rf $(BUILD_DIR) $(DIST_DIR)

iso: $(DIST_ISO)

run: iso
	@qemu-system-i386 -d int,cpu_reset -cdrom $(DIST_ISO)

debug: iso
	@qemu-system-i386 -s -S -d int,cpu_reset -cdrom $(DIST_ISO)

$(DIST_DIR):
	@mkdir -p $@
	@cp -r $(ROOT_DIR)/sysroot-template $(DIST_DIR)/iso

$(DIST_ISO): $(DIST_DIR) kernel
	@grub-mkrescue -o $(DIST_ISO) $(DIST_DIR)/iso

include ./kernel/Makefile
