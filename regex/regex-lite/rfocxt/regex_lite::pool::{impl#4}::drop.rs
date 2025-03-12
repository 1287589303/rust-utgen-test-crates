pub(crate) type CachePool = Pool<pikevm::Cache, CachePoolFn>;
pub(crate) type CachePoolGuard<'a> = PoolGuard<'a, pikevm::Cache, CachePoolFn>;
pub(crate) type CachePoolFn = Box<
    dyn Fn() -> pikevm::Cache + Send + Sync + UnwindSafe + RefUnwindSafe,
>;
use core::panic::{RefUnwindSafe, UnwindSafe};
use alloc::{boxed::Box, vec, vec::Vec};
use crate::pikevm;
use std::sync::Mutex;
pub(crate) struct PoolGuard<'a, T: Send, F: Fn() -> T> {
    /// The pool that this guard is attached to.
    pool: &'a Pool<T, F>,
    /// This is None after the guard has been put back into the pool.
    value: Option<Box<T>>,
}
pub(crate) struct Pool<T, F> {
    /// A stack of T values to hand out. These are used when a Pool is
    /// accessed by a thread that didn't create it.
    stack: Mutex<Vec<Box<T>>>,
    /// A function to create more T values when stack is empty and a caller
    /// has requested a T.
    create: F,
}
impl<'a, T: Send, F: Fn() -> T> Drop for PoolGuard<'a, T, F> {
    fn drop(&mut self) {
        if let Some(value) = self.value.take() {
            self.pool.put_value(value);
        }
    }
}
impl<T: Send, F: Fn() -> T> Pool<T, F> {
    pub(crate) fn get(&self) -> PoolGuard<'_, T, F> {}
    fn put_value(&self, value: Box<T>) {
        let mut stack = self.stack.lock().unwrap();
        stack.push(value);
    }
}
