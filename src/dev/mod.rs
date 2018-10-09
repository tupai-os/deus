#[derive(Clone, Debug)]
pub enum SerialError {}

pub trait SerialOut {
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
