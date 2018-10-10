// Serial

#[derive(Clone, Debug)]
pub enum SerialError {}

pub trait SerialWriter {
    type Item: Copy + Clone;

    fn write_one(&mut self, item: Self::Item) -> Result<(), SerialError>;

    fn write_many(&mut self, items: impl Iterator<Item=Self::Item>) -> Result<(), (usize, SerialError)> {
        // Default implementation
        for (i, item) in items.enumerate() {
            if let Err(err) = self.write_one(item) {
                return Err((i, err));
            }
        }
        Ok(())
    }
}

pub trait SerialReader {
    type Item: Copy + Clone;

    fn read_one(&mut self) -> Result<Option<Self::Item>, SerialError>;

    fn read_many(&mut self, buff: &mut [Self::Item]) -> Result<usize, (usize, SerialError)> {
        // Default implementation
        for i in 0..buff.len() {
            match self.read_one() {
                Ok(Some(item)) => buff.get_mut(i).map(|e| *e = item),
                Ok(None) => return Ok(i),
                Err(err) => return Err((i, err)),
            };
        }
        Ok(buff.len())
    }
}

// Framebuffer

#[derive(Clone, Debug)]
pub enum FramebufferError {
    OutOfBounds,
}

pub trait Framebuffer {
    type Item: Copy + Clone;

    fn get(&self, x: usize, y: usize) -> Result<Self::Item, FramebufferError>;
    fn set(&mut self, x: usize, y: usize, item: Self::Item) -> Result<(), FramebufferError>;
}

// Printer

#[derive(Clone, Debug)]
pub enum PrinterError {
    FramebufferErr(FramebufferError),
    NotPrintable,
}

impl From<FramebufferError> for PrinterError {
    fn from(err: FramebufferError) -> Self { PrinterError::FramebufferErr(err) }
}

pub trait Printer {
    fn write_char(&mut self, c: char) -> Result<(), PrinterError>;
    fn write_str(&mut self, s: &str) -> Result<usize, (usize, PrinterError)> {
        // Default implementation
        for (i, c) in s.chars().enumerate() {
            if let Err(err) = self.write_char(c) {
                return Err((i, err));
            }
        }
        Ok(s.len())
    }
}
