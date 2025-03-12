pub type Locations = CaptureLocations;
use alloc::{borrow::Cow, string::String, sync::Arc};
use regex_automata::{meta, util::captures, Input, PatternID};
use crate::{error::Error, RegexBuilder};
pub trait Replacer {
    fn replace_append(&mut self, caps: &Captures<'_>, dst: &mut String);
    fn no_expansion<'r>(&'r mut self) -> Option<Cow<'r, str>>;
    fn by_ref<'r>(&'r mut self) -> ReplacerRef<'r, Self> {
        ReplacerRef(self)
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match<'h> {
    haystack: &'h str,
    start: usize,
    end: usize,
}
impl<'h> From<Match<'h>> for &'h str {
    fn from(m: Match<'h>) -> &'h str {
        m.as_str()
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
    pub fn as_str(&self) -> &'h str {
        &self.haystack[self.range()]
    }
    #[inline]
    fn new(haystack: &'h str, start: usize, end: usize) -> Match<'h> {}
}
