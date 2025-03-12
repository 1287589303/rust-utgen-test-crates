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
pub struct CaptureMatches<'r, 'h> {
    haystack: &'h str,
    re: &'r Regex,
    it: pikevm::CapturesMatches<'r, 'h>,
}
pub struct Regex {
    pikevm: Arc<PikeVM>,
    pool: CachePool,
}
pub struct Captures<'h> {
    haystack: &'h str,
    slots: CaptureLocations,
    pikevm: Arc<PikeVM>,
}
#[derive(Debug)]
pub(crate) struct CapturesMatches<'r, 'h> {
    it: FindMatches<'r, 'h>,
}
impl<'r, 'h> Iterator for CaptureMatches<'r, 'h> {
    type Item = Captures<'h>;
    #[inline]
    fn next(&mut self) -> Option<Captures<'h>> {}
    #[inline]
    fn count(self) -> usize {
        self.it.count()
    }
}
