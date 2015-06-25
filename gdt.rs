extern crate core;

pub static GDT_ARR: [GDTEntry; 3] = [
    // NULL
    GDTEntry::null(),

    // Kernel code segment
    GDTEntry {
        limit_low: 0xFFFF,
        base_low: 0x0,
        base_middle: 0x0,
    //      Present,   1,         Exec,      Readable
    access: (1 << 7) | (1 << 4) | (1 << 3) | (1 << 1),
    //         Gran,      Size
    lim_flags: (1 << 7) | (1 << 6) | 0xF,
    base_high: 0x0,
    },

    // Kernel data segment
    GDTEntry {
        limit_low: 0xFFFF,
        base_low: 0x0,
        base_middle: 0x0,
    //      Present,   1,         Writable
    access: (1 << 7) | (1 << 4) | (1 << 1),
    //         Gran,      Size
    lim_flags: (1 << 7) | (1 << 6) | 0xF,
    base_high: 0x0,
    }
];

#[repr(C, packed)]
pub struct GDT {
    pub size: u16,
    pub addr: *const GDTEntry, 
}

#[repr(C, packed)]
pub struct GDTEntry {
    pub limit_low: u16,
    pub base_low: u16,
    pub base_middle: u8,
    pub access: u8,
    pub lim_flags: u8,
    pub base_high: u8
}

impl GDTEntry {
    /// Constructs a NULL GDT entry.
    const fn null() -> GDTEntry {
        GDTEntry {
            limit_low: 0, base_low: 0, base_middle: 0, access: 0,
            lim_flags: 0, base_high: 0
        }
    }
}
