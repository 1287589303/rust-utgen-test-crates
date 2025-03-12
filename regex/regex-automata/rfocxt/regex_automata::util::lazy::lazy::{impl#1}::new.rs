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
impl<T, F> Lazy<T, F> {
    pub(super) const fn new(create: F) -> Lazy<T, F> {
        Lazy {
            state: AtomicU8::new(LAZY_STATE_INIT),
            create: Cell::new(Some(create)),
            data: Cell::new(MaybeUninit::uninit()),
        }
    }
}
