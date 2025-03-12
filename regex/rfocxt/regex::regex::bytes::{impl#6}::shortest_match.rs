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
impl Regex {
    #[inline]
    pub fn shortest_match(&self, haystack: &[u8]) -> Option<usize> {
        self.shortest_match_at(haystack, 0)
    }
    #[inline]
    pub fn shortest_match_at(&self, haystack: &[u8], start: usize) -> Option<usize> {
        let input = Input::new(haystack).earliest(true).span(start..haystack.len());
        self.meta.search_half(&input).map(|hm| hm.offset())
    }
    #[inline]
    pub fn is_match_at(&self, haystack: &[u8], start: usize) -> bool {}
    #[inline]
    pub fn find_at<'h>(&self, haystack: &'h [u8], start: usize) -> Option<Match<'h>> {}
    #[inline]
    pub fn captures_at<'h>(
        &self,
        haystack: &'h [u8],
        start: usize,
    ) -> Option<Captures<'h>> {}
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
