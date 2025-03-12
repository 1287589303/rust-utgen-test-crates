use alloc::{
    borrow::Cow, boxed::Box, string::String, string::ToString, sync::Arc, vec, vec::Vec,
};
use crate::{
    error::Error, hir::{self, Hir},
    int::NonMaxUsize, interpolate, nfa::{self, NFA},
    pikevm::{self, Cache, PikeVM},
    pool::CachePool,
};
#[derive(Debug)]
pub struct Matches<'r, 'h> {
    haystack: &'h str,
    it: pikevm::FindMatches<'r, 'h>,
}
#[derive(Debug)]
pub(crate) struct FindMatches<'r, 'h> {
    pikevm: &'r PikeVM,
    cache: CachePoolGuard<'r>,
    haystack: &'h [u8],
    at: usize,
    slots: Vec<Option<NonMaxUsize>>,
    last_match_end: Option<usize>,
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match<'h> {
    haystack: &'h str,
    start: usize,
    end: usize,
}
impl<'r, 'h> Iterator for Matches<'r, 'h> {
    type Item = Match<'h>;
    #[inline]
    fn next(&mut self) -> Option<Match<'h>> {}
    #[inline]
    fn count(self) -> usize {
        self.it.count()
    }
}
