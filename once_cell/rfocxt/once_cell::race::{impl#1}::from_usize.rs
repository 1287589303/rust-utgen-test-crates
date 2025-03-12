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
pub struct OnceBool {
    inner: OnceNonZeroUsize,
}
#[derive(Default, Debug)]
pub struct OnceNonZeroUsize {
    inner: AtomicUsize,
}
impl OnceBool {
    #[inline]
    pub const fn new() -> OnceBool {}
    #[inline]
    pub fn get(&self) -> Option<bool> {}
    #[inline]
    pub fn set(&self, value: bool) -> Result<(), ()> {}
    pub fn get_or_init<F>(&self, f: F) -> bool
    where
        F: FnOnce() -> bool,
    {}
    pub fn get_or_try_init<F, E>(&self, f: F) -> Result<bool, E>
    where
        F: FnOnce() -> Result<bool, E>,
    {}
    #[inline]
    fn from_usize(value: NonZeroUsize) -> bool {
        value.get() == 1
    }
    #[inline]
    fn to_usize(value: bool) -> NonZeroUsize {}
}
