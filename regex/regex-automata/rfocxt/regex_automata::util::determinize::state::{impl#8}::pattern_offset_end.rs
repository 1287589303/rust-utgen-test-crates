use core::mem;
use alloc::{sync::Arc, vec::Vec};
use crate::util::{
    int::{I32, U32},
    look::LookSet, primitives::{PatternID, StateID},
    wire::{self, Endian},
};
struct Repr<'a>(&'a [u8]);
impl<'a> Repr<'a> {
    fn is_match(&self) -> bool {}
    fn has_pattern_ids(&self) -> bool {}
    fn is_from_word(&self) -> bool {}
    fn is_half_crlf(&self) -> bool {}
    fn look_have(&self) -> LookSet {}
    fn look_need(&self) -> LookSet {}
    fn match_len(&self) -> usize {}
    fn match_pattern(&self, index: usize) -> PatternID {}
    fn match_pattern_ids(&self) -> Option<Vec<PatternID>> {}
    fn iter_match_pattern_ids<F: FnMut(PatternID)>(&self, mut f: F) {}
    fn iter_nfa_state_ids<F: FnMut(StateID)>(&self, mut f: F) {}
    fn pattern_offset_end(&self) -> usize {
        let encoded = self.encoded_pattern_len();
        if encoded == 0 {
            return 9;
        }
        encoded.checked_mul(4).unwrap().checked_add(13).unwrap()
    }
    fn encoded_pattern_len(&self) -> usize {
        if !self.has_pattern_ids() {
            return 0;
        }
        usize::try_from(wire::read_u32(&self.0[9..13])).unwrap()
    }
}
