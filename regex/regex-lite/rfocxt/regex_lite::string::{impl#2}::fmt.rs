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
#[derive(Clone, Debug)]
pub(crate) struct PikeVM {
    nfa: NFA,
}
impl core::fmt::Debug for Regex {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Regex").field(&self.as_str()).finish()
    }
}
impl Regex {
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.pikevm.nfa().pattern()
    }
    #[inline]
    pub fn capture_names(&self) -> CaptureNames<'_> {}
    #[inline]
    pub fn captures_len(&self) -> usize {}
    #[inline]
    pub fn static_captures_len(&self) -> Option<usize> {}
    #[inline]
    pub fn capture_locations(&self) -> CaptureLocations {}
}
