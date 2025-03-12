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
    pub(super) fn get(&self) -> &T {}
    fn poll(&self) -> Option<&T> {
        if self.state.load(Ordering::Acquire) == LAZY_STATE_DONE {
            Some(unsafe { &*(*self.data.as_ptr()).as_ptr() })
        } else {
            None
        }
    }
}
