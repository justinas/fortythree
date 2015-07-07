#![crate_type="lib"]
#![feature(no_std, lang_items)]
#![feature(asm)]
#![feature(const_fn)]
#![feature(core, core_slice_ext, core_str_ext)]
#![no_std]

#[macro_use]
extern crate core;
use core::fmt::Write;
use core::mem;

mod gdt;
use gdt::*;
mod idt;
use idt::*;
mod tui;
mod utils;
pub use utils::memcpy;

extern "C" {
    fn load_gdt(addr: &GDT);
    fn load_idt(addr: &IDT);
}

#[no_mangle]
pub extern "C" fn setup_gdt() {
    let gdt_struct = GDT {
        size: (mem::size_of_val(&gdt::GDT_ARR) - 1) as u16,
        addr: &gdt::GDT_ARR as *const GDTEntry };

    unsafe {
        load_gdt(&gdt_struct);
    }
}

#[no_mangle]
pub extern "C" fn setup_idt() {
    unsafe {
        idt::fill_idt();
        let idt_struct = IDT {
            size: (mem::size_of_val(&idt::IDT_ARR) - 1) as u16,
            addr: &idt::IDT_ARR as *const IDTEntry
        };
        load_idt(&idt_struct);
    }
}

#[no_mangle]
pub unsafe extern "C" fn interrupt_handler(interrupt_no: i32, err_code: i32) {
    if interrupt_no == 32 {
        return; // timer, ignore
    } else if interrupt_no == 33 {
        // consume keyboard presses
        asm!("in al, 0x60" : : : "eax" : "intel")
    }

    let con = &mut tui::CONSOLE;
    write!(con, "\nInterrupt no: {}", interrupt_no);
}

/// Called from assembly as soon as possible
#[no_mangle]
pub extern "C" fn kmain() {

    unsafe {
        let mut con = &mut tui::CONSOLE;
        write!(con, "Hello world.\n");
        write!(con, "I'm just testing really long strings to see if linebreaks are applied correctly to them. ");
        write!(con, "What I want to do next is implement some scrolling and after that this console UI will be cool enough.");
    }
}

#[no_mangle]
pub fn __morestack() { }

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
