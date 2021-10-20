include ./config.mk

DIST_ISO := $(DIST_DIR)/z-os.iso

.PHONY: all clean iso run

all: iso

clean:
	@rm -rf $(BUILD_DIR) $(DIST_DIR)

iso: $(DIST_ISO)

run: iso
	@qemu-system-i386 -cdrom $(DIST_ISO)

$(DIST_DIR):
	@mkdir -p $@
	@cp -r $(ROOT_DIR)/sysroot-template $(DIST_DIR)/iso

$(DIST_ISO): $(DIST_DIR) kernel
	@grub-mkrescue -o $(DIST_ISO) $(DIST_DIR)/iso

include ./kernel/Makefile
