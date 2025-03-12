use core::{
    cell::UnsafeCell, panic::{RefUnwindSafe, UnwindSafe},
    sync::atomic::{AtomicBool, Ordering},
};
use alloc::{boxed::Box, vec, vec::Vec};
pub(super) struct PoolGuard<'a, T: Send, F: Fn() -> T> {
    /// The pool that this guard is attached to.
    pool: &'a Pool<T, F>,
    /// This is None after the guard has been put back into the pool.
    value: Option<Box<T>>,
}
pub(super) struct Pool<T, F> {
    /// A stack of T values to hand out. These are used when a Pool is
    /// accessed by a thread that didn't create it.
    stack: Mutex<Vec<Box<T>>>,
    /// A function to create more T values when stack is empty and a caller
    /// has requested a T.
    create: F,
}
pub struct Pool<T, F = fn() -> T>(alloc::boxed::Box<inner::Pool<T, F>>);
impl<'a, T: Send, F: Fn() -> T> Drop for PoolGuard<'a, T, F> {
    #[inline]
    fn drop(&mut self) {
        self.put_imp();
    }
}
impl<'a, T: Send, F: Fn() -> T> PoolGuard<'a, T, F> {
    #[inline]
    pub(super) fn value(&self) -> &T {}
    #[inline]
    pub(super) fn value_mut(&mut self) -> &mut T {}
    #[inline]
    pub(super) fn put(this: PoolGuard<'_, T, F>) {}
    #[inline(always)]
    fn put_imp(&mut self) {
        if let Some(value) = self.value.take() {
            self.pool.put_value(value);
        }
    }
}
