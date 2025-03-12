pub type Locations = CaptureLocations;
use alloc::{borrow::Cow, string::String, sync::Arc};
use regex_automata::{meta, util::captures, Input, PatternID};
use crate::{error::Error, RegexBuilder};
pub struct Captures<'h> {
    haystack: &'h str,
    caps: captures::Captures,
    static_captures_len: Option<usize>,
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match<'h> {
    haystack: &'h str,
    start: usize,
    end: usize,
}
pub struct Captures<'h> {
    haystack: &'h [u8],
    caps: captures::Captures,
    static_captures_len: Option<usize>,
}
impl<'h> Captures<'h> {
    #[inline]
    pub fn get(&self, i: usize) -> Option<Match<'h>> {}
    #[inline]
    pub fn name(&self, name: &str) -> Option<Match<'h>> {
        self.caps
            .get_group_by_name(name)
            .map(|sp| Match::new(self.haystack, sp.start, sp.end))
    }
    pub fn extract<const N: usize>(&self) -> (&'h str, [&'h str; N]) {}
    #[inline]
    pub fn expand(&self, replacement: &str, dst: &mut String) {}
    #[inline]
    pub fn iter<'c>(&'c self) -> SubCaptureMatches<'c, 'h> {}
    #[inline]
    pub fn len(&self) -> usize {}
}
