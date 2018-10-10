// Library
use spin::{RwLock, Once};

// Kernel
use crate::dev::{
    Framebuffer,
    Printer, PrinterError,
};

pub struct Tty {
    cursor: (usize, usize),
    size: (usize, usize),
}

impl Tty {
    const fn singleton() -> Tty {
        Tty {
            cursor: (0, 0),
            size: (80, 25),
        }
    }
}

impl Printer for Tty {
    fn write_char(&mut self, c: char) -> Result<(), PrinterError> {
        // TODO: Make this driver agnostic
        use crate::driver::vga;
        vga::singleton()
            .write()
            .textmode_mut()
            .set(
                self.cursor.0,
                self.cursor.1,
                if c.is_ascii() { (c as u8).into() } else { b'?'.into() },
            )?;

        // Move cursor accordingly
        self.cursor.0 += 1;
        if self.cursor.0 == self.size.0 {
            self.cursor.0 = 0;

            self.cursor.1 += 1;
            if self.cursor.1 == self.size.1 {
                self.cursor.1 = 0;
            }
        }

        Ok(())
    }
}

static DEFAULT: Once<RwLock<Tty>> = Once::new();

pub fn default_do_for_mut<F: FnOnce(&mut Tty)>(f: F) -> Result<(), PrinterError> {
    f(&mut DEFAULT.call_once(|| RwLock::new(Tty::singleton())).write());
    Ok(())
}
