#! /bin/bash

IMAGE_PATH="$1"
if [[ "$IMAGE_PATH" == "" ]]
then
  IMAGE_PATH="target/x86_64-z_os/debug/bootimage-z_os.bin"
fi

qemu-system-x86_64 -drive format=raw,file="$IMAGE_PATH"
