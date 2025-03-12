use core::mem;
use alloc::{sync::Arc, vec::Vec};
use crate::util::{
    int::{I32, U32},
    look::LookSet, primitives::{PatternID, StateID},
    wire::{self, Endian},
};
#[derive(Clone)]
pub(crate) struct StateBuilderMatches(Vec<u8>);
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct LookSet {
    /// The underlying representation this set is exposed to make it possible
    /// to store it somewhere efficiently. The representation is that
    /// of a bitset, where each assertion occupies bit `i` where
    /// `i = Look::as_repr()`.
    ///
    /// Note that users of this internal representation must permit the full
    /// range of `u16` values to be represented. For example, even if the
    /// current implementation only makes use of the 10 least significant bits,
    /// it may use more bits in a future semver compatible release.
    pub bits: u32,
}
impl StateBuilderMatches {
    pub(crate) fn into_nfa(mut self) -> StateBuilderNFA {}
    pub(crate) fn set_is_from_word(&mut self) {}
    pub(crate) fn set_is_half_crlf(&mut self) {}
    pub(crate) fn look_have(&self) -> LookSet {
        LookSet::read_repr(&self.0[1..])
    }
    pub(crate) fn set_look_have(&mut self, set: impl FnMut(LookSet) -> LookSet) {}
    pub(crate) fn add_match_pattern_id(&mut self, pid: PatternID) {}
    fn repr(&self) -> Repr<'_> {}
    fn repr_vec(&mut self) -> ReprVec<'_> {}
}
impl LookSet {
    #[inline]
    pub fn empty() -> LookSet {}
    #[inline]
    pub fn full() -> LookSet {}
    #[inline]
    pub fn singleton(look: Look) -> LookSet {}
    #[inline]
    pub fn len(self) -> usize {}
    #[inline]
    pub fn is_empty(self) -> bool {}
    #[inline]
    pub fn contains(self, look: Look) -> bool {}
    #[inline]
    pub fn contains_anchor(&self) -> bool {}
    #[inline]
    pub fn contains_anchor_haystack(&self) -> bool {}
    #[inline]
    pub fn contains_anchor_line(&self) -> bool {}
    #[inline]
    pub fn contains_anchor_lf(&self) -> bool {}
    #[inline]
    pub fn contains_anchor_crlf(&self) -> bool {}
    #[inline]
    pub fn contains_word(self) -> bool {}
    #[inline]
    pub fn contains_word_unicode(self) -> bool {}
    #[inline]
    pub fn contains_word_ascii(self) -> bool {}
    #[inline]
    pub fn iter(self) -> LookSetIter {}
    #[inline]
    pub fn insert(self, look: Look) -> LookSet {}
    #[inline]
    pub fn set_insert(&mut self, look: Look) {}
    #[inline]
    pub fn remove(self, look: Look) -> LookSet {}
    #[inline]
    pub fn set_remove(&mut self, look: Look) {}
    #[inline]
    pub fn subtract(self, other: LookSet) -> LookSet {}
    #[inline]
    pub fn set_subtract(&mut self, other: LookSet) {}
    #[inline]
    pub fn union(self, other: LookSet) -> LookSet {}
    #[inline]
    pub fn set_union(&mut self, other: LookSet) {}
    #[inline]
    pub fn intersect(self, other: LookSet) -> LookSet {}
    #[inline]
    pub fn set_intersect(&mut self, other: LookSet) {}
    #[inline]
    pub fn read_repr(slice: &[u8]) -> LookSet {
        let bits = u32::from_ne_bytes(slice[..4].try_into().unwrap());
        LookSet { bits }
    }
    #[inline]
    pub fn write_repr(self, slice: &mut [u8]) {}
    pub fn available(self) -> Result<(), UnicodeWordBoundaryError> {}
}
