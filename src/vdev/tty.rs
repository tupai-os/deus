use core::fmt;
use spin::Once;
use crate::util::IrqLock;

// Kernel
use crate::util::ioface::{Framebuffer, IoError, Write};

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

    fn increment_cursor(&mut self, n: usize) {
        // Move cursor accordingly
        self.cursor.0 += n;
        while self.cursor.0 >= self.size.0 {
            self.cursor.0 -= self.size.0;

            self.cursor.1 += 1;
            if self.cursor.1 == self.size.1 {
                self.cursor.1 = 0;
            }
        }
    }
}

impl Write<char> for Tty {
    fn write_one(&mut self, c: char) -> Result<(), IoError> {
        match c {
            '\t' => {
                self.increment_cursor(1);
                while self.cursor.0 % 4 != 0 {
                    self.increment_cursor(1);
                }
                Ok(())
            },
            '\n' => {
                self.cursor.0 = 0;
                self.cursor.1 = (self.cursor.1 + 1) % self.size.1;

                Ok(())
            }
            c if c.is_ascii_graphic() || c == ' ' => {
                // TODO: Make this driver agnostic
                use crate::driver::vga;
                vga::singleton().lock().textmode_mut().set(
                    self.cursor.0,
                    self.cursor.1,
                    if c.is_ascii() {
                        (c as u8).into()
                    } else {
                        b'?'.into()
                    },
                )?;

                self.increment_cursor(1);

                Ok(())
            }
            _ => Err(IoError::InvalidItem),
        }
    }
}

impl fmt::Write for Tty {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        s.chars()
            .try_for_each(|c| self.write_one(c))
            .map_err(|_| fmt::Error::default())
    }
}

static DEFAULT: Once<IrqLock<Tty>> = Once::new();

pub fn default_do_for_mut<R, F: FnOnce(&mut Tty) -> R>(f: F) -> Result<R, IoError> {
    Ok(f(&mut DEFAULT
        .call_once(|| IrqLock::new(Tty::singleton()))
        .lock()))
}
