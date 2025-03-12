pub type Locations = CaptureLocations;
use alloc::{borrow::Cow, string::String, sync::Arc, vec::Vec};
use regex_automata::{meta, util::captures, Input, PatternID};
use crate::{bytes::RegexBuilder, error::Error};
#[derive(Clone)]
pub struct Regex {
    pub(crate) meta: meta::Regex,
    pub(crate) pattern: Arc<str>,
}
#[derive(Clone)]
pub struct Regex {
    pub(crate) meta: meta::Regex,
    pub(crate) pattern: Arc<str>,
}
pub struct Captures<'h> {
    haystack: &'h [u8],
    caps: captures::Captures,
    static_captures_len: Option<usize>,
}
impl Regex {
    #[inline]
    pub fn shortest_match(&self, haystack: &[u8]) -> Option<usize> {}
    #[inline]
    pub fn shortest_match_at(&self, haystack: &[u8], start: usize) -> Option<usize> {}
    #[inline]
    pub fn is_match_at(&self, haystack: &[u8], start: usize) -> bool {}
    #[inline]
    pub fn find_at<'h>(&self, haystack: &'h [u8], start: usize) -> Option<Match<'h>> {}
    #[inline]
    pub fn captures_at<'h>(
        &self,
        haystack: &'h [u8],
        start: usize,
    ) -> Option<Captures<'h>> {
        let input = Input::new(haystack).span(start..haystack.len());
        let mut caps = self.meta.create_captures();
        self.meta.captures(input, &mut caps);
        if caps.is_match() {
            let static_captures_len = self.static_captures_len();
            Some(Captures {
                haystack,
                caps,
                static_captures_len,
            })
        } else {
            None
        }
    }
    #[inline]
    pub fn captures_read<'h>(
        &self,
        locs: &mut CaptureLocations,
        haystack: &'h [u8],
    ) -> Option<Match<'h>> {}
    #[inline]
    pub fn captures_read_at<'h>(
        &self,
        locs: &mut CaptureLocations,
        haystack: &'h [u8],
        start: usize,
    ) -> Option<Match<'h>> {}
    #[inline]
    pub fn read_captures_at<'h>(
        &self,
        locs: &mut CaptureLocations,
        haystack: &'h [u8],
        start: usize,
    ) -> Option<Match<'h>> {}
}
