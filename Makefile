GDB = i386-elf-gdb
LD = i386-elf-ld
RANLIB = i386-elf-ranlib
OBJCOPY = i386-elf-objcopy

# QEMU binary
QEMU ?= qemu-system-i386
QEMU_FLAGS ?= 

RUST_TREE ?= ~/src/rust
# Common flags for our Rust builds
RUST_FLAGS ?= -C target_cpu=i386 --target i686-unknown-linux-gnu -g
# Flags required for Rust to play nice in a bare-bones environment
RUST_FREESTANDING_FLAGS ?= -C no-stack-check -Z force-overflow-checks=off

kernel: link.ld interrupts.o loader.o libkernel.rlib
	$(LD) -o $@ -T $^
	$(OBJCOPY) --only-keep-debug $@ $@.sym

%.o: %.s
	nasm -f elf32 -g -o $@ $^

libkernel.rlib: kernel.rs gdt.rs idt.rs tui.rs libcore.rlib
	rustc -L. $(RUST_FLAGS) $(RUST_FREESTANDING_FLAGS) -C opt-level=0 kernel.rs
	# For some reason, when producing an rlib
	# rustc produces symbols with zero addresses.
	# Ranlib fixes this.
	$(RANLIB) $@

libcore.rlib:
	rustc $(RUST_FLAGS) $(RUST_FREESTANDING_FLAGS) -C opt-level=3 $(RUST_TREE)/src/libcore/lib.rs

run: kernel
	$(QEMU) $(QEMU_FLAGS) -kernel $<

debug: QEMU_FLAGS += -S -s
debug: run

gdb: kernel
	$(GDB) -x gdbinit

clean:
	rm -f libkernel.rlib *.o *.sym
