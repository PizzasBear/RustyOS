#!/bin/sh

cargo bootimage --target=rustyos.json
qemu-system-x86_64 -drive format=raw,file="target/rustyos/debug/bootimage-rustyos.bin"
