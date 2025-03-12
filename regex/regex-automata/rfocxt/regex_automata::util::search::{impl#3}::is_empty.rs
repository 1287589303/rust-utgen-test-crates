use core::ops::{Range, RangeBounds};
use crate::util::{escape::DebugByte, primitives::PatternID, utf8};
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Span {
    /// The start offset of the span, inclusive.
    pub start: usize,
    /// The end offset of the span, exclusive.
    pub end: usize,
}
impl Span {
    #[inline]
    pub fn range(&self) -> Range<usize> {}
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.start >= self.end
    }
    #[inline]
    pub fn len(&self) -> usize {}
    #[inline]
    pub fn contains(&self, offset: usize) -> bool {}
    #[inline]
    pub fn offset(&self, offset: usize) -> Span {}
}
