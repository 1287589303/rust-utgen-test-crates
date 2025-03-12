use core::mem;
use alloc::{sync::Arc, vec::Vec};
use crate::util::{
    int::{I32, U32},
    look::LookSet, primitives::{PatternID, StateID},
    wire::{self, Endian},
};
pub(crate) trait Endian {
    fn write_u16(n: u16, dst: &mut [u8]);
    fn write_u32(n: u32, dst: &mut [u8]);
    fn write_u128(n: u128, dst: &mut [u8]);
}
struct ReprVec<'a>(&'a mut Vec<u8>);
struct Repr<'a>(&'a [u8]);
impl<'a> ReprVec<'a> {
    fn set_is_match(&mut self) {}
    fn set_has_pattern_ids(&mut self) {}
    fn set_is_from_word(&mut self) {}
    fn set_is_half_crlf(&mut self) {}
    fn look_have(&self) -> LookSet {}
    fn look_need(&self) -> LookSet {}
    fn set_look_have(&mut self, mut set: impl FnMut(LookSet) -> LookSet) {}
    fn set_look_need(&mut self, mut set: impl FnMut(LookSet) -> LookSet) {}
    fn add_match_pattern_id(&mut self, pid: PatternID) {}
    fn close_match_pattern_ids(&mut self) {
        if !self.repr().has_pattern_ids() {
            return;
        }
        let patsize = PatternID::SIZE;
        let pattern_bytes = self.0.len() - 13;
        assert_eq!(pattern_bytes % patsize, 0);
        let count32 = u32::try_from(pattern_bytes / patsize).unwrap();
        wire::NE::write_u32(count32, &mut self.0[9..13]);
    }
    fn add_nfa_state_id(&mut self, prev: &mut StateID, sid: StateID) {}
    fn repr(&self) -> Repr<'_> {
        Repr(self.0.as_slice())
    }
}
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
    fn match_pattern(&self, index: usize) -> PatternID {}
    fn match_pattern_ids(&self) -> Option<Vec<PatternID>> {}
    fn iter_match_pattern_ids<F: FnMut(PatternID)>(&self, mut f: F) {}
    fn iter_nfa_state_ids<F: FnMut(StateID)>(&self, mut f: F) {}
    fn pattern_offset_end(&self) -> usize {}
    fn encoded_pattern_len(&self) -> usize {}
}
