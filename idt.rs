use core::slice::SliceExt;

const INTERRUPT_COUNT: usize = 48; // 32 exceptions + IRQ0-7 + IRQ8-15

// To be initialized by fill_idt()
pub static mut IDT_ARR: [IDTEntry; INTERRUPT_COUNT] = [IDTEntry::null(); INTERRUPT_COUNT];

#[repr(C, packed)]
pub struct IDT {
    pub size: u16,
    pub addr: *const IDTEntry,
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct IDTEntry {
    offset_low: u16,
    selector: u16,
    zero: u8,
    attr: u8,
    offset_high: u16,
}

pub unsafe fn fill_idt() {
    let mut i = 0;
    let len = IDT_ARR.len();

    while i < len {
        let ptr = IDT_ARR.get_unchecked_mut(i);
        *ptr = IDTEntry {
            offset_low: ((*ALL_HANDLERS.get_unchecked(i) as usize) & 0xFFFF) as u16,
            selector: 0x08,
            zero: 0x0,
            attr: match i {
                // Trap
                3 | 4 => (1 << 7) | 0xF,
                // Interrupt
                _ => (1 << 7) | 0xE
            },
            offset_high: ((*ALL_HANDLERS.get_unchecked(i) as usize) >> 16) as u16,
        };
        i += 1;
    }


}

impl IDTEntry {
    const fn null() -> IDTEntry {
        IDTEntry {
            offset_low: 0x0,
            selector: 0x0,
            zero: 0x0,
            attr: 0x0,
            offset_high: 0x0,
        }
    }
}

// Mess
extern {
    fn interrupt_handler_0();
    fn interrupt_handler_1();
    fn interrupt_handler_2();
    fn interrupt_handler_3();
    fn interrupt_handler_4();
    fn interrupt_handler_5();
    fn interrupt_handler_6();
    fn interrupt_handler_7();
    fn interrupt_handler_8();
    fn interrupt_handler_9();
    fn interrupt_handler_10();
    fn interrupt_handler_11();
    fn interrupt_handler_12();
    fn interrupt_handler_13();
    fn interrupt_handler_14();
    fn interrupt_handler_15();
    fn interrupt_handler_16();
    fn interrupt_handler_17();
    fn interrupt_handler_18();
    fn interrupt_handler_19();
    fn interrupt_handler_20();
    fn interrupt_handler_21();
    fn interrupt_handler_22();
    fn interrupt_handler_23();
    fn interrupt_handler_24();
    fn interrupt_handler_25();
    fn interrupt_handler_26();
    fn interrupt_handler_27();
    fn interrupt_handler_28();
    fn interrupt_handler_29();
    fn interrupt_handler_30();
    fn interrupt_handler_31();
    fn interrupt_handler_32();
    fn interrupt_handler_33();
    fn interrupt_handler_34();
    fn interrupt_handler_35();
    fn interrupt_handler_36();
    fn interrupt_handler_37();
    fn interrupt_handler_38();
    fn interrupt_handler_39();
    fn interrupt_handler_40();
    fn interrupt_handler_41();
    fn interrupt_handler_42();
    fn interrupt_handler_43();
    fn interrupt_handler_44();
    fn interrupt_handler_45();
    fn interrupt_handler_46();
    fn interrupt_handler_47();
}

const ALL_HANDLERS: [*const (); INTERRUPT_COUNT] = [
    interrupt_handler_0 as *const (),
    interrupt_handler_1 as *const (),
    interrupt_handler_2 as *const (),
    interrupt_handler_3 as *const (),
    interrupt_handler_4 as *const (),
    interrupt_handler_5 as *const (),
    interrupt_handler_6 as *const (),
    interrupt_handler_7 as *const (),
    interrupt_handler_8 as *const (),
    interrupt_handler_9 as *const (),
    interrupt_handler_10 as *const (),
    interrupt_handler_11 as *const (),
    interrupt_handler_12 as *const (),
    interrupt_handler_13 as *const (),
    interrupt_handler_14 as *const (),
    interrupt_handler_15 as *const (),
    interrupt_handler_16 as *const (),
    interrupt_handler_17 as *const (),
    interrupt_handler_18 as *const (),
    interrupt_handler_19 as *const (),
    interrupt_handler_20 as *const (),
    interrupt_handler_21 as *const (),
    interrupt_handler_22 as *const (),
    interrupt_handler_23 as *const (),
    interrupt_handler_24 as *const (),
    interrupt_handler_25 as *const (),
    interrupt_handler_26 as *const (),
    interrupt_handler_27 as *const (),
    interrupt_handler_28 as *const (),
    interrupt_handler_29 as *const (),
    interrupt_handler_30 as *const (),
    interrupt_handler_31 as *const (),
    interrupt_handler_32 as *const (),
    interrupt_handler_33 as *const (),
    interrupt_handler_34 as *const (),
    interrupt_handler_35 as *const (),
    interrupt_handler_36 as *const (),
    interrupt_handler_37 as *const (),
    interrupt_handler_38 as *const (),
    interrupt_handler_39 as *const (),
    interrupt_handler_40 as *const (),
    interrupt_handler_41 as *const (),
    interrupt_handler_42 as *const (),
    interrupt_handler_43 as *const (),
    interrupt_handler_44 as *const (),
    interrupt_handler_45 as *const (),
    interrupt_handler_46 as *const (),
    interrupt_handler_47 as *const (),
];
