#!/bin/sh

cargo bootimage --target=rustyos.json --release
qemu-system-x86_64 -drive format=raw,file="target/rustyos/release/bootimage-rustyos.bin"
