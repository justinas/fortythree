GDB = i386-elf-gdb
LD = i386-elf-ld
OBJCOPY = i386-elf-objcopy

# QEMU binary
QEMU ?= qemu-system-i386
QEMU_FLAGS ?= 

RUST_TREE ?= ~/src/rust
# Common flags for our Rust builds
RUST_FLAGS ?= -C target_cpu=i386 --target i686-unknown-linux-gnu -g
# Flags required for Rust to play nice in a bare-bones environment
RUST_FREESTANDING_FLAGS ?= -C no-stack-check -Z force-overflow-checks=off

kernel: link.ld loader.o kernel.o 
	$(LD) -o $@ -T $^
	$(OBJCOPY) --only-keep-debug $@ $@.sym

loader.o: loader.s
	nasm -f elf32 -g -o $@ $^

kernel.o: kernel.rs gdt.rs tui.rs libcore.rlib
	rustc -L. $(RUST_FLAGS) $(RUST_FREESTANDING_FLAGS) -C opt-level=0 --emit obj kernel.rs

libcore.rlib:
	rustc $(RUST_FLAGS) $(RUST_FREESTANDING_FLAGS) -C opt-level=3 $(RUST_TREE)/src/libcore/lib.rs

run: kernel
	$(QEMU) $(QEMU_FLAGS) -kernel $<

debug: QEMU_FLAGS += -S -s
debug: run

gdb: kernel
	$(GDB) -x gdbinit

clean:
	rm -f kernel *.o *.sym
