use core::{
    ptr::NonNull,
    mem::MaybeUninit,
};
use alloc::alloc::{GlobalAlloc, Layout};
use linked_list_allocator::Heap as InnerHeap;
use crate::util::IrqLock;

pub struct Heap {
    inner: IrqLock<MaybeUninit<InnerHeap>>,
}

impl Heap {
    pub const fn uninit() -> Self {
        Self {
            inner: IrqLock::new(MaybeUninit::uninit()),
        }
    }

    pub unsafe fn init(&mut self, region: &'static mut [u8]) {
        self
            .inner
            .lock()
            .write(InnerHeap::new(
                (region as *mut _ as *const u8) as usize,
                region.len(),
            ));
    }
}

unsafe impl GlobalAlloc for Heap {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self
            .inner
            .lock()
            .get_mut()
            .allocate_first_fit(layout)
            .map(|ptr| ptr.as_ptr())
            .unwrap_or(core::ptr::null_mut())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self
            .inner
            .lock()
            .get_mut()
            .deallocate(NonNull::new_unchecked(ptr), layout);
    }
}

unsafe impl Send for Heap {}
unsafe impl Sync for Heap {}
