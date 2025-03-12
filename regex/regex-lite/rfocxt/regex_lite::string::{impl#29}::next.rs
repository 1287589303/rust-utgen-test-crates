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
pub struct Captures<'h> {
    haystack: &'h str,
    slots: CaptureLocations,
    pikevm: Arc<PikeVM>,
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match<'h> {
    haystack: &'h str,
    start: usize,
    end: usize,
}
#[derive(Clone, Debug)]
pub struct CaptureNames<'r>(nfa::CaptureNames<'r>);
#[derive(Clone, Debug)]
pub(crate) struct CaptureNames<'a> {
    it: core::slice::Iter<'a, Option<Arc<str>>>,
}
impl<'c, 'h> Iterator for SubCaptureMatches<'c, 'h> {
    type Item = Option<Match<'h>>;
    #[inline]
    fn next(&mut self) -> Option<Option<Match<'h>>> {
        let (group_index, _) = self.it.next()?;
        Some(self.caps.get(group_index))
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {}
    #[inline]
    fn count(self) -> usize {}
}
impl<'h> Captures<'h> {
    #[inline]
    pub fn get(&self, i: usize) -> Option<Match<'h>> {
        self.slots.get(i).map(|(s, e)| Match::new(self.haystack, s, e))
    }
    #[inline]
    pub fn name(&self, name: &str) -> Option<Match<'h>> {}
    pub fn extract<const N: usize>(&self) -> (&'h str, [&'h str; N]) {}
    #[inline]
    pub fn expand(&self, replacement: &str, dst: &mut String) {}
    #[inline]
    pub fn iter<'c>(&'c self) -> SubCaptureMatches<'c, 'h> {}
    #[inline]
    pub fn len(&self) -> usize {}
}
