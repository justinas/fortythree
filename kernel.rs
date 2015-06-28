#![crate_type="staticlib"]
#![feature(no_std, lang_items)]
#![feature(const_fn)]
#![feature(core, core_slice_ext)]
#![no_std]

extern crate core;
use core::mem::size_of_val;

mod gdt;
use gdt::*;
mod tui;
mod utils;
pub use utils::memcpy;

extern "C" {
    fn load_gdt(addr: &GDT);
}

fn setup_gdt() {
    let gdt_struct = GDT {
        size: (size_of_val(&gdt::GDT_ARR) - 1) as u16,
        addr: &gdt::GDT_ARR as *const GDTEntry };

    unsafe {
        load_gdt(&gdt_struct);
    }
}

/// Called from assembly as soon as possible
#[no_mangle]
pub extern "C" fn kmain() {
    setup_gdt();

    unsafe {
        let mut con = &mut tui::CONSOLE;
        con.write_str("Hello world.\n");
        con.write_str("I'm just testing really long strings to see if linebreaks are applied correctly to them. ");
        con.write_str("What I want to do next is implement some scrolling and after that this console UI will be cool enough.");
    }
}

#[no_mangle]
pub fn __morestack() { }

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
