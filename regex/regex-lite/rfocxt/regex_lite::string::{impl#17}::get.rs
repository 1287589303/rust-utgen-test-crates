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
pub struct CaptureLocations(Vec<Option<NonMaxUsize>>);
#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub(crate) struct NonMaxUsize(NonZeroUsize);
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
impl NonMaxUsize {
    pub(crate) fn new(value: usize) -> Option<NonMaxUsize> {}
    pub(crate) fn get(self) -> usize {
        self.0.get().wrapping_sub(1)
    }
}
