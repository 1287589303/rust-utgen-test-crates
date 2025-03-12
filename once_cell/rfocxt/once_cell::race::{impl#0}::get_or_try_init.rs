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
    pub unsafe fn get_unchecked(&self) -> NonZeroUsize {}
    #[inline]
    pub fn set(&self, value: NonZeroUsize) -> Result<(), ()> {}
    pub fn get_or_init<F>(&self, f: F) -> NonZeroUsize
    where
        F: FnOnce() -> NonZeroUsize,
    {}
    pub fn get_or_try_init<F, E>(&self, f: F) -> Result<NonZeroUsize, E>
    where
        F: FnOnce() -> Result<NonZeroUsize, E>,
    {
        let val = self.inner.load(Ordering::Acquire);
        match NonZeroUsize::new(val) {
            Some(it) => Ok(it),
            None => self.init(f),
        }
    }
    #[cold]
    #[inline(never)]
    fn init<E>(
        &self,
        f: impl FnOnce() -> Result<NonZeroUsize, E>,
    ) -> Result<NonZeroUsize, E> {
        let mut val = f()?.get();
        let exchange = self
            .inner
            .compare_exchange(0, val, Ordering::AcqRel, Ordering::Acquire);
        if let Err(old) = exchange {
            val = old;
        }
        Ok(unsafe { NonZeroUsize::new_unchecked(val) })
    }
}
