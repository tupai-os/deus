// Library
use core::slice;
use spin::RwLock;

// Kernel
use crate::dev::{SerialOut, SerialError};

const TEXTMODE_BUFFER: *mut [u8; 2] = 0xB8000 as *mut [u8; 2];
const TEXTMODE_COLS: usize = 80;
const TEXTMODE_ROWS: usize = 25;

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

impl SerialOut for Textmode {
    type Item = Char;

    fn write_one(&mut self, c: Char) -> Result<(), SerialError> {
        let buffer_size = TEXTMODE_COLS * TEXTMODE_ROWS;
        let buffer = unsafe { slice::from_raw_parts_mut(TEXTMODE_BUFFER, buffer_size) };
        buffer.get_mut(self.cursor % buffer_size).map(|cell| *cell = [c.ascii, c.color]);
        self.cursor += 1;
        Ok(())
    }
}

pub struct Vga {
    textmode: Textmode,
}

impl Vga {
    const fn singleton() -> Vga {
        Vga {
            textmode: Textmode {
                cursor: 0,
            },
        }
    }

    pub fn textmode(&self) -> &Textmode {
        &self.textmode
    }

    pub fn textmode_mut(&mut self) -> &mut Textmode {
        &mut self.textmode
    }
}

pub static SINGLETON: RwLock<Vga> = RwLock::new(Vga::singleton());
