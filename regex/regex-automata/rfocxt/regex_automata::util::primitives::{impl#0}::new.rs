use core::num::NonZeroUsize;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use crate::util::int::{Usize, U16, U32, U64};
#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct NonMaxUsize(NonZeroUsize);
impl NonMaxUsize {
    #[inline]
    pub fn new(value: usize) -> Option<NonMaxUsize> {
        NonZeroUsize::new(value.wrapping_add(1)).map(NonMaxUsize)
    }
    #[inline]
    pub fn get(self) -> usize {}
}
