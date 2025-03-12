use alloc::{
    borrow::Cow, boxed::Box, string::String, string::ToString, sync::Arc, vec, vec::Vec,
};
use crate::{
    error::Error, hir::{self, Hir},
    int::NonMaxUsize, interpolate, nfa::{self, NFA},
    pikevm::{self, Cache, PikeVM},
    pool::CachePool,
};
pub struct Captures<'h> {
    haystack: &'h str,
    slots: CaptureLocations,
    pikevm: Arc<PikeVM>,
}
#[derive(Clone, Debug)]
pub struct CaptureLocations(Vec<Option<NonMaxUsize>>);
#[derive(Clone, Debug)]
pub(crate) struct PikeVM {
    nfa: NFA,
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match<'h> {
    haystack: &'h str,
    start: usize,
    end: usize,
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
impl CaptureLocations {
    #[inline]
    pub fn get(&self, i: usize) -> Option<(usize, usize)> {
        let slot = i.checked_mul(2)?;
        let start = self.0.get(slot).copied()??.get();
        let slot = slot.checked_add(1)?;
        let end = self.0.get(slot).copied()??.get();
        Some((start, end))
    }
    #[inline]
    pub fn len(&self) -> usize {}
}
