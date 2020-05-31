arch ?= x86_64
kernel := target/build/kernel-$(arch).bin
iso := target/build/os-$(arch).iso
target ?= rustyos-$(arch)
rust_os := target/$(target)/debug/librustyos.a

linker_script := src/arch/$(arch)/linker.ld
grub_cfg := src/arch/$(arch)/grub.cfg
assembly_source_files := $(wildcard src/arch/$(arch)/*.asm)
assembly_object_files := $(patsubst src/arch/$(arch)/%.asm, \
	target/build/arch/$(arch)/%.o, $(assembly_source_files))

.PHONY: all clean run iso

all: $(kernel)

clean:
	@rm -r target/build

run: $(iso)
	@qemu-system-x86_64 -cdrom $(iso)

iso: $(iso)

$(iso): $(kernel) $(grub_cfg)
	@mkdir -p target/build/isofiles/boot/grub
	@cp $(kernel) target/build/isofiles/boot/kernel.bin
	@cp $(grub_cfg) target/build/isofiles/boot/grub
	@grub-mkrescue -o $(iso) target/build/isofiles 2> /dev/null
	@rm -r target/build/isofiles

$(kernel): kernel $(rust_os) $(assembly_object_files) $(linker_script)
	@ld -n -T $(linker_script) -o $(kernel) $(assembly_object_files) $(rust_os)

kernel:
	@cargo xbuild --target $(target).json

# compile assembly files
target/build/arch/$(arch)/%.o: src/arch/$(arch)/%.asm
	@mkdir -p $(shell dirname $@)
	@nasm -felf64 $< -o $@
