use core::{cmp, mem, num::NonZeroUsize};
use alloc::{vec, vec::Vec};
use crate::hir::{self, Hir};
#[derive(Clone, Eq, PartialEq, PartialOrd, Ord)]
pub struct Literal {
    bytes: Vec<u8>,
    exact: bool,
}
pub(crate) struct Bytes<'a>(pub(crate) &'a [u8]);
impl core::fmt::Debug for Literal {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let tag = if self.exact { "E" } else { "I" };
        f.debug_tuple(tag).field(&crate::debug::Bytes(self.as_bytes())).finish()
    }
}
impl Literal {
    #[inline]
    pub fn exact<B: Into<Vec<u8>>>(bytes: B) -> Literal {}
    #[inline]
    pub fn inexact<B: Into<Vec<u8>>>(bytes: B) -> Literal {}
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {}
    #[inline]
    pub fn len(&self) -> usize {}
    #[inline]
    pub fn is_empty(&self) -> bool {}
    #[inline]
    pub fn is_exact(&self) -> bool {}
    #[inline]
    pub fn make_inexact(&mut self) {}
    #[inline]
    pub fn reverse(&mut self) {}
    #[inline]
    pub fn extend(&mut self, lit: &Literal) {}
    #[inline]
    pub fn keep_first_bytes(&mut self, len: usize) {}
    #[inline]
    pub fn keep_last_bytes(&mut self, len: usize) {}
    fn is_poisonous(&self) -> bool {}
}
