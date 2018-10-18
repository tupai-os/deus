// Library
use core::slice;
use spin::RwLock;
use volatile::Volatile;

// Kernel
use crate::util::ioface::{Framebuffer, IoError, Write};

const TEXTMODE_BUFFER: *mut Volatile<Char> = 0xB8000 as *mut Volatile<Char>;
const TEXTMODE_COLS: usize = 80;
const TEXTMODE_ROWS: usize = 25;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Char {
    ascii: u8,
    color: u8,
}

// A neat hack to make printing byte strings easier. TODO: remove this?
impl From<u8> for Char {
    fn from(ascii: u8) -> Char {
        Char {
            ascii,
            color: 0x0F, // TODO: Define this color in a more sensible way
        }
    }
}

pub struct Textmode {
    cursor: usize,
}

impl Textmode {
    fn buffer(&self) -> &'static mut [Volatile<Char>] {
        unsafe { slice::from_raw_parts_mut(TEXTMODE_BUFFER, TEXTMODE_COLS * TEXTMODE_ROWS) }
    }
}

impl Write<Char> for Textmode {
    fn write_one(&mut self, c: Char) -> Result<(), IoError> {
        self.buffer()
            .get_mut(self.cursor % self.buffer().len())
            .map(|cell| cell.write(c));
        self.cursor += 1;
        Ok(())
    }
}

impl Framebuffer<Char> for Textmode {
    fn get(&self, x: usize, y: usize) -> Result<Char, IoError> {
        self.buffer()
            .get(y * TEXTMODE_COLS + x)
            .map(|e| e.read())
            .ok_or(IoError::OutOfBounds)
    }

    fn set(&mut self, x: usize, y: usize, c: Char) -> Result<(), IoError> {
        self.buffer()
            .get_mut(y * TEXTMODE_COLS + x)
            .map(|e| e.write(c))
            .ok_or(IoError::OutOfBounds)
    }
}

pub struct Vga {
    textmode: Textmode,
}

impl Vga {
    const fn singleton() -> Vga {
        Vga {
            textmode: Textmode { cursor: 0 },
        }
    }

    pub fn textmode(&self) -> &Textmode {
        &self.textmode
    }

    pub fn textmode_mut(&mut self) -> &mut Textmode {
        &mut self.textmode
    }
}

static SINGLETON: RwLock<Vga> = RwLock::new(Vga::singleton());

pub fn singleton() -> &'static RwLock<Vga> {
    &SINGLETON
}
