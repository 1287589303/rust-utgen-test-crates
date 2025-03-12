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
    fn is_match(&self) -> bool {
        self.0[0] & (1 << 0) > 0
    }
    fn has_pattern_ids(&self) -> bool {
        self.0[0] & (1 << 1) > 0
    }
    fn is_from_word(&self) -> bool {}
    fn is_half_crlf(&self) -> bool {}
    fn look_have(&self) -> LookSet {}
    fn look_need(&self) -> LookSet {}
    fn match_len(&self) -> usize {}
    fn match_pattern(&self, index: usize) -> PatternID {}
    fn match_pattern_ids(&self) -> Option<Vec<PatternID>> {}
    fn iter_match_pattern_ids<F: FnMut(PatternID)>(&self, mut f: F) {
        if !self.is_match() {
            return;
        }
        if !self.has_pattern_ids() {
            f(PatternID::ZERO);
            return;
        }
        let mut pids = &self.0[13..self.pattern_offset_end()];
        while !pids.is_empty() {
            let pid = wire::read_u32(pids);
            pids = &pids[PatternID::SIZE..];
            f(PatternID::new_unchecked(usize::try_from(pid).unwrap()));
        }
    }
    fn iter_nfa_state_ids<F: FnMut(StateID)>(&self, mut f: F) {}
    fn pattern_offset_end(&self) -> usize {
        let encoded = self.encoded_pattern_len();
        if encoded == 0 {
            return 9;
        }
        encoded.checked_mul(4).unwrap().checked_add(13).unwrap()
    }
    fn encoded_pattern_len(&self) -> usize {}
}
#[cfg_attr(feature = "perf-inline", inline(always))]
pub(crate) fn read_u32(slice: &[u8]) -> u32 {
    let bytes: [u8; 4] = slice[..size_of::<u32>()].try_into().unwrap();
    u32::from_ne_bytes(bytes)
}
