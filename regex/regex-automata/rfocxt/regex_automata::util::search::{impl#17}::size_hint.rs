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
impl<'a> Iterator for PatternSetIter<'a> {
    type Item = PatternID;
    fn next(&mut self) -> Option<PatternID> {}
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.it.size_hint()
    }
}
