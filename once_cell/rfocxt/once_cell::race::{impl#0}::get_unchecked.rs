#[cfg(not(feature = "portable-atomic"))]
use core::sync::atomic;
#[cfg(feature = "portable-atomic")]
use portable_atomic as atomic;
use atomic::{AtomicPtr, AtomicUsize, Ordering};
use core::cell::UnsafeCell;
use core::marker::PhantomData;
use core::num::NonZeroUsize;
use core::ptr;
#[cfg(feature = "alloc")]
pub use self::once_box::OnceBox;
#[derive(Default, Debug)]
pub struct OnceNonZeroUsize {
    inner: AtomicUsize,
}
impl OnceNonZeroUsize {
    #[inline]
    pub const fn new() -> OnceNonZeroUsize {}
    #[inline]
    pub fn get(&self) -> Option<NonZeroUsize> {}
    pub unsafe fn get_unchecked(&self) -> NonZeroUsize {
        let p = self.inner.as_ptr();
        let val = unsafe { p.read() };
        unsafe { NonZeroUsize::new_unchecked(val) }
    }
    #[inline]
    pub fn set(&self, value: NonZeroUsize) -> Result<(), ()> {}
    pub fn get_or_init<F>(&self, f: F) -> NonZeroUsize
    where
        F: FnOnce() -> NonZeroUsize,
    {}
    pub fn get_or_try_init<F, E>(&self, f: F) -> Result<NonZeroUsize, E>
    where
        F: FnOnce() -> Result<NonZeroUsize, E>,
    {}
    #[cold]
    #[inline(never)]
    fn init<E>(
        &self,
        f: impl FnOnce() -> Result<NonZeroUsize, E>,
    ) -> Result<NonZeroUsize, E> {}
}
