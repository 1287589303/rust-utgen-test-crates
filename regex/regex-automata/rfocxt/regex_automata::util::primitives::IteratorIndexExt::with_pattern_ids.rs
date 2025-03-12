use core::num::NonZeroUsize;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use crate::util::int::{Usize, U16, U32, U64};
pub(crate) trait IteratorIndexExt: Iterator {
    fn with_pattern_ids(self) -> WithPatternIDIter<Self>
    where
        Self: Sized + ExactSizeIterator,
    {
        WithPatternIDIter::new(self)
    }
    fn with_state_ids(self) -> WithStateIDIter<Self>
    where
        Self: Sized + ExactSizeIterator,
    {
        WithStateIDIter::new(self)
    }
}
