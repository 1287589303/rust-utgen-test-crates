use core::{
    cell::UnsafeCell, panic::{RefUnwindSafe, UnwindSafe},
    sync::atomic::{AtomicUsize, Ordering},
};
use alloc::{boxed::Box, vec, vec::Vec};
use std::{sync::Mutex, thread_local};
static COUNTER: AtomicUsize = AtomicUsize::new(3);
static THREAD_ID_UNOWNED: usize = 0;
static THREAD_ID_INUSE: usize = 1;
static THREAD_ID_DROPPED: usize = 2;
const MAX_POOL_STACKS: usize = 8;
pub(super) struct Pool<T, F> {
    /// A stack of T values to hand out. These are used when a Pool is
    /// accessed by a thread that didn't create it.
    stack: Mutex<Vec<Box<T>>>,
    /// A function to create more T values when stack is empty and a caller
    /// has requested a T.
    create: F,
}
pub(super) struct PoolGuard<'a, T: Send, F: Fn() -> T> {
    /// The pool that this guard is attached to.
    pool: &'a Pool<T, F>,
    /// This is None after the guard has been put back into the pool.
    value: Option<Box<T>>,
}
#[derive(Debug)]
struct Mutex<T> {
    locked: AtomicBool,
    data: UnsafeCell<T>,
}
impl<T: Send, F: Fn() -> T> Pool<T, F> {
    #[inline]
    pub(super) fn get(&self) -> PoolGuard<'_, T, F> {}
    #[cold]
    fn get_slow(&self, caller: usize, owner: usize) -> PoolGuard<'_, T, F> {}
    #[inline]
    fn put_value(&self, value: Box<T>) {}
    #[inline]
    fn guard_owned(&self, caller: usize) -> PoolGuard<'_, T, F> {}
    #[inline]
    fn guard_stack(&self, value: Box<T>) -> PoolGuard<'_, T, F> {}
    #[inline]
    fn guard_stack_transient(&self, value: Box<T>) -> PoolGuard<'_, T, F> {
        PoolGuard {
            pool: self,
            value: Ok(value),
            discard: true,
        }
    }
}
