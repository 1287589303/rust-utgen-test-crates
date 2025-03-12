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
#[derive(Debug)]
pub struct Split<'r, 'h> {
    haystack: &'h str,
    finder: Matches<'r, 'h>,
    last: usize,
}
#[derive(Debug)]
pub struct Matches<'r, 'h> {
    haystack: &'h str,
    it: pikevm::FindMatches<'r, 'h>,
}
#[derive(Clone, Debug)]
pub(crate) struct PikeVM {
    nfa: NFA,
}
impl Regex {
    pub fn new(pattern: &str) -> Result<Regex, Error> {}
    #[inline]
    pub fn is_match(&self, haystack: &str) -> bool {}
    #[inline]
    pub fn find<'h>(&self, haystack: &'h str) -> Option<Match<'h>> {}
    #[inline]
    pub fn find_iter<'r, 'h>(&'r self, haystack: &'h str) -> Matches<'r, 'h> {
        Matches {
            haystack,
            it: self.pikevm.find_iter(self.pool.get(), haystack.as_bytes()),
        }
    }
    #[inline]
    pub fn captures<'h>(&self, haystack: &'h str) -> Option<Captures<'h>> {}
    #[inline]
    pub fn captures_iter<'r, 'h>(&'r self, haystack: &'h str) -> CaptureMatches<'r, 'h> {}
    #[inline]
    pub fn split<'r, 'h>(&'r self, haystack: &'h str) -> Split<'r, 'h> {
        Split {
            haystack,
            finder: self.find_iter(haystack),
            last: 0,
        }
    }
    #[inline]
    pub fn splitn<'r, 'h>(&'r self, haystack: &'h str, limit: usize) -> SplitN<'r, 'h> {}
    #[inline]
    pub fn replace<'h, R: Replacer>(&self, haystack: &'h str, rep: R) -> Cow<'h, str> {}
    #[inline]
    pub fn replace_all<'h, R: Replacer>(
        &self,
        haystack: &'h str,
        rep: R,
    ) -> Cow<'h, str> {}
    #[inline]
    pub fn replacen<'h, R: Replacer>(
        &self,
        haystack: &'h str,
        limit: usize,
        mut rep: R,
    ) -> Cow<'h, str> {}
}
