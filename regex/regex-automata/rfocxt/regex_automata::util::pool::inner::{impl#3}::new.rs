use core::{
    cell::UnsafeCell, panic::{RefUnwindSafe, UnwindSafe},
    sync::atomic::{AtomicBool, Ordering},
};
use alloc::{boxed::Box, vec, vec::Vec};
pub(super) struct Pool<T, F> {
    /// A stack of T values to hand out. These are used when a Pool is
    /// accessed by a thread that didn't create it.
    stack: Mutex<Vec<Box<T>>>,
    /// A function to create more T values when stack is empty and a caller
    /// has requested a T.
    create: F,
}
#[derive(Debug)]
struct Mutex<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}
impl<T, F> Pool<T, F> {
    pub(super) const fn new(create: F) -> Pool<T, F> {
        Pool {
            stack: Mutex::new(vec![]),
            create,
        }
    }
}
