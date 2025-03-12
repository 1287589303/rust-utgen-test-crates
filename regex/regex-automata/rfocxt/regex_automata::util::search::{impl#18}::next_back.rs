use core::ops::{Range, RangeBounds};
use crate::util::{escape::DebugByte, primitives::PatternID, utf8};
#[cfg(feature = "alloc")]
#[derive(Clone, Debug)]
pub struct PatternSetIter<'a> {
    it: core::iter::Enumerate<core::slice::Iter<'a, bool>>,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
#[cfg(feature = "alloc")]
impl<'a> DoubleEndedIterator for PatternSetIter<'a> {
    fn next_back(&mut self) -> Option<PatternID> {
        while let Some((index, &yes)) = self.it.next_back() {
            if yes {
                return Some(PatternID::new_unchecked(index));
            }
        }
        None
    }
}
