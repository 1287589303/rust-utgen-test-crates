use core::ops::{Range, RangeBounds};
use crate::util::{escape::DebugByte, primitives::PatternID, utf8};
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Match {
    /// The pattern ID.
    pattern: PatternID,
    /// The underlying match span.
    span: Span,
}
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Span {
    /// The start offset of the span, inclusive.
    pub start: usize,
    /// The end offset of the span, exclusive.
    pub end: usize,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
impl Match {
    #[inline]
    pub fn new<S: Into<Span>>(pattern: PatternID, span: S) -> Match {}
    #[inline]
    pub fn must<S: Into<Span>>(pattern: usize, span: S) -> Match {}
    #[inline]
    pub fn pattern(&self) -> PatternID {}
    #[inline]
    pub fn start(&self) -> usize {}
    #[inline]
    pub fn end(&self) -> usize {}
    #[inline]
    pub fn range(&self) -> core::ops::Range<usize> {}
    #[inline]
    pub fn span(&self) -> Span {
        self.span
    }
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.span().is_empty()
    }
    #[inline]
    pub fn len(&self) -> usize {}
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
