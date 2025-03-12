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
pub struct CaptureNames<'r>(nfa::CaptureNames<'r>);
#[derive(Clone, Debug)]
pub(crate) struct CaptureNames<'a> {
    it: core::slice::Iter<'a, Option<Arc<str>>>,
}
impl<'r> Iterator for CaptureNames<'r> {
    type Item = Option<&'r str>;
    #[inline]
    fn next(&mut self) -> Option<Option<&'r str>> {
        self.0.next()
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {}
    #[inline]
    fn count(self) -> usize {}
}
