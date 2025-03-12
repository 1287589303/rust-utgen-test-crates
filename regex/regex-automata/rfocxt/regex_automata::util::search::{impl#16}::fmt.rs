use core::ops::{Range, RangeBounds};
use crate::util::{escape::DebugByte, primitives::PatternID, utf8};
#[cfg(feature = "alloc")]
#[derive(Clone, Debug)]
pub struct PatternSetInsertError {
    attempted: PatternID,
    capacity: usize,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
#[cfg(feature = "alloc")]
impl core::fmt::Display for PatternSetInsertError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "failed to insert pattern ID {} into pattern set \
             with insufficiet capacity of {}",
            self.attempted.as_usize(), self.capacity,
        )
    }
}
