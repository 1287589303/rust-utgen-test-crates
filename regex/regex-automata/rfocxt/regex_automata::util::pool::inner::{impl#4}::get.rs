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
pub(super) struct PoolGuard<'a, T: Send, F: Fn() -> T> {
    /// The pool that this guard is attached to.
    pool: &'a Pool<T, F>,
    /// This is None after the guard has been put back into the pool.
    value: Option<Box<T>>,
}
impl<T: Send, F: Fn() -> T> Pool<T, F> {
    #[inline]
    pub(super) fn get(&self) -> PoolGuard<'_, T, F> {
        let mut stack = self.stack.lock();
        let value = match stack.pop() {
            None => Box::new((self.create)()),
            Some(value) => value,
        };
        PoolGuard {
            pool: self,
            value: Some(value),
        }
    }
    #[inline]
    fn put(&self, guard: PoolGuard<'_, T, F>) {}
    #[inline]
    fn put_value(&self, value: Box<T>) {}
}
