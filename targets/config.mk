include $(realpath $(dir $(abspath $(lastword $(MAKEFILE_LIST)))))/$(ARCH).mk

ifeq (,$(wildcard ./$(RUST_TARGET)))
	RUST_TARGET := $(realpath $(dir $(abspath $(lastword $(MAKEFILE_LIST)))))/$(RUST_TARGET)
endif
