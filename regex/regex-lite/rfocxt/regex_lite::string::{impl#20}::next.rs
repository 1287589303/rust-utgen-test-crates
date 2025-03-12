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
#[derive(Debug)]
pub(crate) struct CapturesMatches<'r, 'h> {
    it: FindMatches<'r, 'h>,
}
#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub(crate) struct NonMaxUsize(NonZeroUsize);
pub struct Captures<'h> {
    haystack: &'h str,
    slots: CaptureLocations,
    pikevm: Arc<PikeVM>,
}
pub struct Regex {
    pikevm: Arc<PikeVM>,
    pool: CachePool,
}
impl<'r, 'h> Iterator for CaptureMatches<'r, 'h> {
    type Item = Captures<'h>;
    #[inline]
    fn next(&mut self) -> Option<Captures<'h>> {
        self.it
            .next()
            .map(|slots| Captures {
                haystack: self.haystack,
                slots: CaptureLocations(slots),
                pikevm: Arc::clone(&self.re.pikevm),
            })
    }
    #[inline]
    fn count(self) -> usize {}
}
