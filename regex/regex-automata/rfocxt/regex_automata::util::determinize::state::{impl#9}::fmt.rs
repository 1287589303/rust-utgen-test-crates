use core::mem;
use alloc::{sync::Arc, vec::Vec};
use crate::util::{
    int::{I32, U32},
    look::LookSet, primitives::{PatternID, StateID},
    wire::{self, Endian},
};
struct Repr<'a>(&'a [u8]);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
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
impl<'a> core::fmt::Debug for Repr<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let mut nfa_ids = alloc::vec![];
        self.iter_nfa_state_ids(|sid| nfa_ids.push(sid));
        f.debug_struct("Repr")
            .field("is_match", &self.is_match())
            .field("is_from_word", &self.is_from_word())
            .field("is_half_crlf", &self.is_half_crlf())
            .field("look_have", &self.look_have())
            .field("look_need", &self.look_need())
            .field("match_pattern_ids", &self.match_pattern_ids())
            .field("nfa_state_ids", &nfa_ids)
            .finish()
    }
}
impl<'a> Repr<'a> {
    fn is_match(&self) -> bool {
        self.0[0] & (1 << 0) > 0
    }
    fn has_pattern_ids(&self) -> bool {}
    fn is_from_word(&self) -> bool {
        self.0[0] & (1 << 2) > 0
    }
    fn is_half_crlf(&self) -> bool {
        self.0[0] & (1 << 3) > 0
    }
    fn look_have(&self) -> LookSet {
        LookSet::read_repr(&self.0[1..])
    }
    fn look_need(&self) -> LookSet {
        LookSet::read_repr(&self.0[5..])
    }
    fn match_len(&self) -> usize {}
    fn match_pattern(&self, index: usize) -> PatternID {}
    fn match_pattern_ids(&self) -> Option<Vec<PatternID>> {
        if !self.is_match() {
            return None;
        }
        let mut pids = alloc::vec![];
        self.iter_match_pattern_ids(|pid| pids.push(pid));
        Some(pids)
    }
    fn iter_match_pattern_ids<F: FnMut(PatternID)>(&self, mut f: F) {}
    fn iter_nfa_state_ids<F: FnMut(StateID)>(&self, mut f: F) {}
    fn pattern_offset_end(&self) -> usize {}
    fn encoded_pattern_len(&self) -> usize {}
}
