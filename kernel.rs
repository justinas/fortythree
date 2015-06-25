#![crate_type="staticlib"]
#![feature(no_std, lang_items)]
#![feature(const_fn)]
#![feature(core, core_str_ext)]
#![no_std]

extern crate core;
use core::mem::size_of_val;
use core::str::StrExt;

mod gdt;
use gdt::*;

const VID_START: usize = 0xb8000;
static HELLO: &'static str = "Hello Rust Kernel.";

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

fn clear_screen() {
    let mut cursor = VID_START as *mut u8;
    for _ in 0..80*25 {
        unsafe {
            *cursor = ' ' as u8;
            cursor = cursor.offset(1);
            *cursor = 0x00;
            cursor = cursor.offset(1);
        }
    }
}

/// Called from assembly as soon as possible
#[no_mangle]
pub extern "C" fn kmain() {
    setup_gdt();
    clear_screen();

    for i in 0..25 {
        unsafe {
            // skip to the required line
            let mut cursor = (VID_START as *mut u8).offset(i * 80 * 2);
            for b in HELLO.bytes() {
                    *cursor = b;
                    cursor = cursor.offset(1);
                    *cursor = 0x07;
                    cursor = cursor.offset(1);

                // delay 
                for _ in 0..1<<15 {
                }
            }
        }
    }
}

#[no_mangle]
pub fn __morestack() { }

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] fn panic_fmt() -> ! { loop {} }
