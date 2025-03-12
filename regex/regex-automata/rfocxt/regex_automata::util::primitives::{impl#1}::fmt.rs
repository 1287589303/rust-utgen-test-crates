use core::num::NonZeroUsize;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use crate::util::int::{Usize, U16, U32, U64};
#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct NonMaxUsize(NonZeroUsize);
impl core::fmt::Debug for NonMaxUsize {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self.get())
    }
}
impl NonMaxUsize {
    #[inline]
    pub fn new(value: usize) -> Option<NonMaxUsize> {}
    #[inline]
    pub fn get(self) -> usize {
        self.0.get().wrapping_sub(1)
    }
}
