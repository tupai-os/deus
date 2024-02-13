use core::mem::MaybeUninit;
use alloc::{
    collections::VecDeque,
    boxed::Box,
};
use crate::{
    hal::StackFrame,
    util::{Store, Id, IrqLock},
    logln,
};

static mut SCHEDULER: MaybeUninit<IrqLock<Scheduler>> = MaybeUninit::uninit();
static mut THREADS: MaybeUninit<IrqLock<Store<Thread>>> = MaybeUninit::uninit();

#[repr(C)]
#[repr(align(16))]
#[derive(Copy, Clone, Default)]
pub struct StackByte { _unused: [u8; 16] }

pub struct ThreadCtx {
    id: Id<Thread>,
    stack: *mut [StackByte],
    frame: *mut StackFrame,
}

#[derive(Default)]
pub struct Scheduler {
    current: Option<ThreadCtx>,
    waiting: VecDeque<ThreadCtx>,
}

pub struct Thread;

pub fn init_threading() {
    unsafe { SCHEDULER.write(IrqLock::new(Scheduler::default())); }
    unsafe { THREADS.write(IrqLock::new(Store::default())); }
}

pub fn spawn_thread<F: FnOnce()>(f: F) -> Id<Thread> {
    fn gen_thread<F: FnOnce()>(f: *mut F) {
        (*unsafe { Box::from_raw(f) })();
        loop { /* TODO: Kill old threads */ }
    }

    let id = unsafe { THREADS.assume_init_mut() }.lock().insert(Thread);

    let stack_size = 4096 * 32;
    let stack = Box::into_raw(vec![StackByte::default(); stack_size / 16].into_boxed_slice());
    let stack_end = unsafe { (stack as *mut u8).offset(stack_size as isize) };
    let frame = unsafe { stack_end.offset(-(core::mem::size_of::<StackFrame>() as isize)) as *mut StackFrame };

    unsafe { frame.write(StackFrame::new_thread(gen_thread::<F>, Box::into_raw(Box::new(f)), stack_end)) };

    unsafe { SCHEDULER.assume_init_mut() }
        .lock()
        .waiting
        .push_back(ThreadCtx { id, stack, frame });
    id
}

pub fn preempt(frame: *mut StackFrame) -> *mut StackFrame {
    let mut scheduler = unsafe { SCHEDULER.assume_init_mut() }.lock();

    if let Some(mut ctx) = scheduler.current.take() {
        ctx.frame = frame;
        scheduler.waiting.push_back(ctx);
    }

    scheduler.current = scheduler.waiting.pop_front();

    if let Some(ctx) = &scheduler.current {
        ctx.frame
    } else {
        frame
    }
}
