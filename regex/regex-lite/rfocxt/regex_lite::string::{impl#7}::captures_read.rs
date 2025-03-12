use alloc::{
    borrow::Cow, boxed::Box, string::String, string::ToString, sync::Arc, vec, vec::Vec,
};
use crate::{
    error::Error, hir::{self, Hir},
    int::NonMaxUsize, interpolate, nfa::{self, NFA},
    pikevm::{self, Cache, PikeVM},
    pool::CachePool,
};
pub struct Regex {
    pikevm: Arc<PikeVM>,
    pool: CachePool,
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match<'h> {
    haystack: &'h str,
    start: usize,
    end: usize,
}
#[derive(Clone, Debug)]
pub(crate) struct PikeVM {
    nfa: NFA,
}
#[derive(Clone, Debug)]
pub struct CaptureLocations(Vec<Option<NonMaxUsize>>);
impl Regex {
    #[inline]
    pub fn shortest_match(&self, haystack: &str) -> Option<usize> {}
    #[inline]
    pub fn shortest_match_at(&self, haystack: &str, start: usize) -> Option<usize> {}
    #[inline]
    pub fn is_match_at(&self, haystack: &str, start: usize) -> bool {}
    #[inline]
    pub fn find_at<'h>(&self, haystack: &'h str, start: usize) -> Option<Match<'h>> {}
    #[inline]
    pub fn captures_at<'h>(
        &self,
        haystack: &'h str,
        start: usize,
    ) -> Option<Captures<'h>> {}
    #[inline]
    pub fn captures_read<'h>(
        &self,
        locs: &mut CaptureLocations,
        haystack: &'h str,
    ) -> Option<Match<'h>> {
        self.captures_read_at(locs, haystack, 0)
    }
    #[inline]
    pub fn captures_read_at<'h>(
        &self,
        locs: &mut CaptureLocations,
        haystack: &'h str,
        start: usize,
    ) -> Option<Match<'h>> {
        let mut cache = self.pool.get();
        let matched = self
            .pikevm
            .search(
                &mut cache,
                haystack.as_bytes(),
                start,
                haystack.len(),
                false,
                &mut locs.0,
            );
        if !matched {
            return None;
        }
        let (start, end) = locs.get(0).unwrap();
        Some(Match::new(haystack, start, end))
    }
}
