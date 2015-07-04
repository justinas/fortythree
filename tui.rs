use core::fmt;
use core::result::Result;
use core::result::Result::*;
use core::mem;
use core::slice::SliceExt;
use core::str::StrExt;

const MEM_START: *mut u8 = 0xb8000 as *mut u8;
const DEFAULT_COLOR: u8 = 0x07;

pub static mut CONSOLE: Console = Console::new();

/// A low-level implementation of vga text mode
struct VGA;

impl VGA {
    pub unsafe fn put_char(&self, ch: u8, color: u8, (x, y) : (u8, u8)) {
        let ptr = self.coord_offset((x, y));
        *ptr = ch;
        *ptr.offset(1) = color;
    }

    fn coord_offset(&self, (x, y): (u8, u8)) -> *mut u8 {
        unsafe {
            MEM_START.offset((x as isize)*2 + (y as isize)*80*2)
        }
    }
}


/// A higher-level implementation of a "console".
pub struct Console {
    // a 2d-array of (char, color) pairs
    buffer: [[(u8, u8); 80]; 25],
    // (x, y) cursor
    x: u8,
    y: u8
}

impl Console {
    const fn new() -> Console {
        Console {
            buffer: [[(0, 0); 80]; 25],
            x: 0,
            y: 0,
        }
    }

    unsafe fn flush(&self) {
        let vga = VGA;
        for y in 0..25 {
            for x in 0..80 {
                let buf_val = *self.buffer.get_unchecked(y as usize).get_unchecked(x as usize);
                vga.put_char(buf_val.0, buf_val.1, (x, y));
            }
        }
    }

    /// Scrolls one line in the buffer,
    /// making the 25-th line free for new text.
    pub fn scroll(&mut self) {
        unsafe {
            for i in 0..24 {
                *self.buffer.get_unchecked_mut(i) = *self.buffer.get_unchecked(i+1)
            }
            *self.buffer.get_unchecked_mut(24) = [(0, 0); 80];
        }
    }

    pub fn write(&mut self, buf: &[u8]) {
        for &b in buf {
            if self.x >= 80 {
                self.x = 0;
                self.y += 1;
            }
            if self.y >= 25 {
                self.scroll();
                self.y = 24;
                self.x = 0;
            }
            match b {
                0x0A => {
                    self.x = 0;
                    self.y += 1;
                },
                _ => unsafe {
                    let row = self.buffer.get_unchecked_mut(self.y as usize);
                    *row.get_unchecked_mut(self.x as usize) = (b, DEFAULT_COLOR);
                    self.x += 1;
                }
            }
        }
        unsafe {
            self.flush();
        }
    }

}

impl fmt::Write for Console {
    fn write_str(&mut self, buf: &str) -> fmt::Result {
        self.write(buf.as_bytes());
        Ok(())
    }
}
