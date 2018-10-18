#[derive(Clone, Debug)]
pub enum IoError {
    InvalidItem,
    OutOfBounds,
}

// Write

pub trait Write<T: Clone> {
    fn write_one(&mut self, item: T) -> Result<(), IoError>;
    fn write_all(&mut self, buf: &[T]) -> Result<usize, IoError> {
        // Default impl
        buf.iter().enumerate().try_fold(buf.len(), |_, (i, item)| self.write_one(item.clone()).map(|_| i))
    }
    fn flush(&mut self) -> Result<(), IoError> { Ok(()) }
}

// Buffer

pub trait Buffer<T: Clone> {
    // TODO
}

// Framebuffer

pub trait Framebuffer<T: Clone> {
    fn get(&self, x: usize, y: usize) -> Result<T, IoError>;
    fn set(&mut self, x: usize, y: usize, item: T) -> Result<(), IoError>;
}
