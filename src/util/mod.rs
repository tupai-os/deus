pub mod ioface;
pub mod store;
pub mod irq_lock;

// Reexports
pub use self::{
    store::{Id, Store},
    irq_lock::IrqLock,
};

use volatile::Volatile;

pub fn wait_cycles(cycles: usize) {
    let mut counter = Volatile::new(0);
    (0..cycles).for_each(|_| counter.write(counter.read() + 1));
}
