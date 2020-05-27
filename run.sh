#!/bin/sh

cargo bootimage --target=tstos.json
qemu-system-x86_64 -drive format=raw,file="target/tstos/debug/bootimage-rustyos.bin"
