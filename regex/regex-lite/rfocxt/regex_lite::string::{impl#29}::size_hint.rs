use alloc::{
    borrow::Cow, boxed::Box, string::String, string::ToString, sync::Arc, vec, vec::Vec,
};
use crate::{
    error::Error, hir::{self, Hir},
    int::NonMaxUsize, interpolate, nfa::{self, NFA},
    pikevm::{self, Cache, PikeVM},
    pool::CachePool,
};
#[derive(Clone, Debug)]
pub struct SubCaptureMatches<'c, 'h> {
    caps: &'c Captures<'h>,
    it: core::iter::Enumerate<nfa::CaptureNames<'c>>,
}
#[derive(Clone, Debug)]
pub struct CaptureNames<'r>(nfa::CaptureNames<'r>);
pub struct Captures<'h> {
    haystack: &'h str,
    slots: CaptureLocations,
    pikevm: Arc<PikeVM>,
}
#[derive(Clone, Debug)]
pub(crate) struct CaptureNames<'a> {
    it: core::slice::Iter<'a, Option<Arc<str>>>,
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match<'h> {
    haystack: &'h str,
    start: usize,
    end: usize,
}
impl<'c, 'h> Iterator for SubCaptureMatches<'c, 'h> {
    type Item = Option<Match<'h>>;
    #[inline]
    fn next(&mut self) -> Option<Option<Match<'h>>> {}
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.it.size_hint()
    }
    #[inline]
    fn count(self) -> usize {}
}
