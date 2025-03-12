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
pub struct PatternID(SmallIndex);
impl<'a> Repr<'a> {
    fn is_match(&self) -> bool {}
    fn has_pattern_ids(&self) -> bool {
        self.0[0] & (1 << 1) > 0
    }
    fn is_from_word(&self) -> bool {}
    fn is_half_crlf(&self) -> bool {}
    fn look_have(&self) -> LookSet {}
    fn look_need(&self) -> LookSet {}
    fn match_len(&self) -> usize {}
    fn match_pattern(&self, index: usize) -> PatternID {
        if !self.has_pattern_ids() {
            PatternID::ZERO
        } else {
            let offset = 13 + index * PatternID::SIZE;
            wire::read_pattern_id_unchecked(&self.0[offset..]).0
        }
    }
    fn match_pattern_ids(&self) -> Option<Vec<PatternID>> {}
    fn iter_match_pattern_ids<F: FnMut(PatternID)>(&self, mut f: F) {}
    fn iter_nfa_state_ids<F: FnMut(StateID)>(&self, mut f: F) {}
    fn pattern_offset_end(&self) -> usize {}
    fn encoded_pattern_len(&self) -> usize {}
}
pub(crate) fn read_pattern_id_unchecked(slice: &[u8]) -> (PatternID, usize) {
    let pid = PatternID::from_ne_bytes_unchecked(
        slice[..PatternID::SIZE].try_into().unwrap(),
    );
    (pid, PatternID::SIZE)
}
