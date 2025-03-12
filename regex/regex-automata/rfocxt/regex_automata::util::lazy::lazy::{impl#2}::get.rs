use core::{
    cell::Cell, fmt, mem::MaybeUninit, panic::{RefUnwindSafe, UnwindSafe},
    sync::atomic::{AtomicU8, Ordering},
};
const LAZY_STATE_INIT: u8 = 0;
const LAZY_STATE_BUSY: u8 = 1;
const LAZY_STATE_DONE: u8 = 2;
pub(super) struct Lazy<T, F> {
    state: AtomicU8,
    create: Cell<Option<F>>,
    data: Cell<MaybeUninit<T>>,
}
impl<T, F: FnOnce() -> T> Lazy<T, F> {
    pub(super) fn get(&self) -> &T {
        while self.state.load(Ordering::Acquire) != LAZY_STATE_DONE {
            let result = self
                .state
                .compare_exchange(
                    LAZY_STATE_INIT,
                    LAZY_STATE_BUSY,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                );
            if let Ok(_) = result {
                let create = unsafe {
                    (*self.create.as_ptr())
                        .take()
                        .expect(
                            "Lazy's create function panicked, \
                             preventing initialization,
                             poisoning current thread",
                        )
                };
                let guard = Guard { state: &self.state };
                unsafe {
                    (*self.data.as_ptr()).as_mut_ptr().write(create());
                }
                core::mem::forget(guard);
                self.state.store(LAZY_STATE_DONE, Ordering::Release);
                break;
            }
            core::hint::spin_loop();
        }
        self.poll().unwrap()
    }
    fn poll(&self) -> Option<&T> {
        if self.state.load(Ordering::Acquire) == LAZY_STATE_DONE {
            Some(unsafe { &*(*self.data.as_ptr()).as_ptr() })
        } else {
            None
        }
    }
}
