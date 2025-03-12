use core::mem;
use alloc::{sync::Arc, vec::Vec};
use crate::util::{
    int::{I32, U32},
    look::LookSet, primitives::{PatternID, StateID},
    wire::{self, Endian},
};
struct ReprVec<'a>(&'a mut Vec<u8>);
impl<'a> ReprVec<'a> {
    fn set_is_match(&mut self) {
        self.0[0] |= 1 << 0;
    }
    fn set_has_pattern_ids(&mut self) {}
    fn set_is_from_word(&mut self) {}
    fn set_is_half_crlf(&mut self) {}
    fn look_have(&self) -> LookSet {}
    fn look_need(&self) -> LookSet {}
    fn set_look_have(&mut self, mut set: impl FnMut(LookSet) -> LookSet) {}
    fn set_look_need(&mut self, mut set: impl FnMut(LookSet) -> LookSet) {}
    fn add_match_pattern_id(&mut self, pid: PatternID) {}
    fn close_match_pattern_ids(&mut self) {}
    fn add_nfa_state_id(&mut self, prev: &mut StateID, sid: StateID) {}
    fn repr(&self) -> Repr<'_> {}
}
