pub type Locations = CaptureLocations;
use alloc::{borrow::Cow, string::String, sync::Arc, vec::Vec};
use regex_automata::{meta, util::captures, Input, PatternID};
use crate::{bytes::RegexBuilder, error::Error};
#[derive(Clone)]
pub struct Regex {
    pub(crate) meta: meta::Regex,
    pub(crate) pattern: Arc<str>,
}
#[derive(Debug)]
pub struct CaptureMatches<'r, 'h> {
    haystack: &'h [u8],
    it: meta::CapturesMatches<'r, 'h>,
}
#[derive(Clone)]
pub struct Regex {
    pub(crate) meta: meta::Regex,
    pub(crate) pattern: Arc<str>,
}
impl Regex {
    pub fn new(re: &str) -> Result<Regex, Error> {}
    #[inline]
    pub fn is_match(&self, haystack: &[u8]) -> bool {}
    #[inline]
    pub fn find<'h>(&self, haystack: &'h [u8]) -> Option<Match<'h>> {}
    #[inline]
    pub fn find_iter<'r, 'h>(&'r self, haystack: &'h [u8]) -> Matches<'r, 'h> {}
    #[inline]
    pub fn captures<'h>(&self, haystack: &'h [u8]) -> Option<Captures<'h>> {}
    #[inline]
    pub fn captures_iter<'r, 'h>(
        &'r self,
        haystack: &'h [u8],
    ) -> CaptureMatches<'r, 'h> {
        CaptureMatches {
            haystack,
            it: self.meta.captures_iter(haystack),
        }
    }
    #[inline]
    pub fn split<'r, 'h>(&'r self, haystack: &'h [u8]) -> Split<'r, 'h> {}
    #[inline]
    pub fn splitn<'r, 'h>(&'r self, haystack: &'h [u8], limit: usize) -> SplitN<'r, 'h> {}
    #[inline]
    pub fn replace<'h, R: Replacer>(&self, haystack: &'h [u8], rep: R) -> Cow<'h, [u8]> {}
    #[inline]
    pub fn replace_all<'h, R: Replacer>(
        &self,
        haystack: &'h [u8],
        rep: R,
    ) -> Cow<'h, [u8]> {}
    #[inline]
    pub fn replacen<'h, R: Replacer>(
        &self,
        haystack: &'h [u8],
        limit: usize,
        mut rep: R,
    ) -> Cow<'h, [u8]> {}
}
