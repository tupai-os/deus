use core::{
    ops::{Deref, DerefMut},
    cell::UnsafeCell,
};
use crate::hal;

/// An IRQ-safe Mutex
pub struct IrqLock<T> {
    inner: UnsafeCell<T>,
}

unsafe impl<T> Send for IrqLock<T> {}
unsafe impl<T> Sync for IrqLock<T> {}

impl<T> IrqLock<T> {
    pub const fn new(inner: T) -> Self {
        Self {
            inner: UnsafeCell::new(inner),
        }
    }

    pub fn lock(&self) -> IrqLockGuard<T> {
        let irqs_enabled = hal::irqs_enabled();
        if irqs_enabled {
            unsafe { hal::disable_irqs(); }
        }

        IrqLockGuard {
            inner: &self.inner,
            reenable: irqs_enabled,
        }
    }

    pub unsafe fn get(&self) -> &T {
        &*self.inner.get()
    }

    pub unsafe fn get_mut(&self) -> &mut T {
        &mut *self.inner.get()
    }
}

pub struct IrqLockGuard<'a, T> {
    inner: &'a UnsafeCell<T>,
    reenable: bool,
}

impl<'a, T> Drop for IrqLockGuard<'a, T> {
    fn drop(&mut self) {
        if self.reenable {
            unsafe { hal::enable_irqs(); }
        }
    }
}

impl<'a, T> Deref for IrqLockGuard<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        unsafe { &*self.inner.get() }
    }
}

impl<'a, T> DerefMut for IrqLockGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.inner.get() }
    }
}
