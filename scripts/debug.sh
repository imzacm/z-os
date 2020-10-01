#! /bin/bash

WAIT_FOR_DEBUG="$1"

if [[ "$WAIT_FOR_DEBUG" == "" ]]
then
  WAIT_FOR_DEBUG=""
else
  WAIT_FOR_DEBUG="-s -S"
fi

cargo bootimage
qemu-system-x86_64 $WAIT_FOR_DEBUG -drive format=raw,file="target/x86_64-z_os/debug/bootimage-z_os.bin" -serial stdio
