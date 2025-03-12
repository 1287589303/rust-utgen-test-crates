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
impl<T, F> Drop for Lazy<T, F> {
    fn drop(&mut self) {
        if *self.state.get_mut() == LAZY_STATE_DONE {
            unsafe {
                self.data.get_mut().assume_init_drop();
            }
        }
    }
}
