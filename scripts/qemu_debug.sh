#! /bin/bash

qemu-system-x86_64 -s -S -drive format=raw,file="target/x86_64-z_os/debug/bootimage-z_os.bin" -serial stdio
