# Resolve an absolute path to the project root (directory containing kernel, libc, rust-toolchain.toml, etc)
ROOT_DIR := $(realpath $(dir $(abspath $(lastword $(MAKEFILE_LIST)))))

## Build configuration
# Target architecture, currently only i686 is supported
ARCH ?= i686
# Build profile, options are "debug" or "release"
PROFILE ?= debug
# Cross-compiler prefix, leave as default if toolchain was built using "setup-toolchain.sh"
PREFIX ?= $(ROOT_DIR)/toolchain/prefix
# Directory to build object files in
BUILD_DIR := $(ROOT_DIR)/build
# Directory to "install" final artifacts
DIST_DIR := $(ROOT_DIR)/dist

## Auto variables, only change if you know what you're doing
# Cross-compiler target, prefixed to command e.g. "gcc" becomes "$(TARGET)-gcc"
TARGET := $(ARCH)-elf
# Name of rust target
RUST_TARGET_NAME := $(ARCH)-unknown-z_os
# The target passed to cargo e.g. "cargo build --target $(RUST_TARGET)"
RUST_TARGET := $(ROOT_DIR)/targets/$(RUST_TARGET_NAME).json
# Path to workspace Cargo.toml
RUST_MANIFEST_PATH := $(ROOT_DIR)/Cargo.toml

# Command used to compile rust
CARGO := cargo
# Command used to compile .asm files
ASM := $(PREFIX)/bin/nasm
# Command used to compile .S files
AS := $(PREFIX)/bin/$(TARGET)-as
# Command used to compile .c files
CC := $(PREFIX)/bin/$(TARGET)-gcc
# Command used to compile .cpp files
CXX := $(PREFIX)/bin/$(TARGET)-g++
# Command used to link the final binary
LD := $(PREFIX)/bin/$(TARGET)-gcc

# Flags passed to $(CARGO)
CARGO_FLAGS := --manifest-path $(RUST_MANIFEST_PATH) --target $(RUST_TARGET)
ifeq ($(PROFILE), release)
CARGO_FLAGS += --release
endif
# Flags passed to $(ASM)
ASM_FLAGS := -felf32
# Flags passed to $(AS)
AS_FLAGS :=
# Flags passed to $(CC)
CC_FLAGS := -O2 -g -ffreestanding -Wall -Wextra -nostdlib -m32 -c
# Flags passed to $(CXX)
CXX_FLAGS := -ffreestanding -O2 -Wall -Wextra -fno-exceptions -fno-rtti
# Flags passed to $(LD)
LD_FLAGS := -O2 -g -ffreestanding -Wall -Wextra -nostdlib -m32
