# Target architecture, currently only x86 is supported
ARCH ?= x86
# Build profile, options are "debug" or "release"
PROFILE ?= debug
# Cross-compiler prefix, leave as default if toolchain was built using "setup-toolchain.sh"
PREFIX ?= $(ROOT_DIR)/../z-os-toolchain/toolchain/prefix

DIST_DIR ?= dist
BUILD_DIR ?= build
