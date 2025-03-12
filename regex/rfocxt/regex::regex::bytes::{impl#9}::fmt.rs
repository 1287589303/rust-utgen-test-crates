pub type Locations = CaptureLocations;
use alloc::{borrow::Cow, string::String, sync::Arc, vec::Vec};
use regex_automata::{meta, util::captures, Input, PatternID};
use crate::{bytes::RegexBuilder, error::Error};
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match<'h> {
    haystack: &'h [u8],
    start: usize,
    end: usize,
}
impl<'h> core::fmt::Debug for Match<'h> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        use regex_automata::util::escape::DebugHaystack;
        let mut fmt = f.debug_struct("Match");
        fmt.field("start", &self.start)
            .field("end", &self.end)
            .field("bytes", &DebugHaystack(&self.as_bytes()));
        fmt.finish()
    }
}
impl<'h> Match<'h> {
    #[inline]
    pub fn start(&self) -> usize {}
    #[inline]
    pub fn end(&self) -> usize {}
    #[inline]
    pub fn is_empty(&self) -> bool {}
    #[inline]
    pub fn len(&self) -> usize {}
    #[inline]
    pub fn range(&self) -> core::ops::Range<usize> {}
    #[inline]
    pub fn as_bytes(&self) -> &'h [u8] {
        &self.haystack[self.range()]
    }
    #[inline]
    fn new(haystack: &'h [u8], start: usize, end: usize) -> Match<'h> {}
}
