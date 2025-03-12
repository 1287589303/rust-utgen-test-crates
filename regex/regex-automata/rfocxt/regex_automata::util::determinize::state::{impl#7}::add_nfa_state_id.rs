use core::mem;
use alloc::{sync::Arc, vec::Vec};
use crate::util::{
    int::{I32, U32},
    look::LookSet, primitives::{PatternID, StateID},
    wire::{self, Endian},
};
#[derive(Clone)]
pub(crate) struct StateBuilderNFA {
    repr: Vec<u8>,
    prev_nfa_state_id: StateID,
}
struct ReprVec<'a>(&'a mut Vec<u8>);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
impl StateBuilderNFA {
    pub(crate) fn to_state(&self) -> State {}
    pub(crate) fn clear(self) -> StateBuilderEmpty {}
    pub(crate) fn look_need(&self) -> LookSet {}
    pub(crate) fn set_look_have(&mut self, set: impl FnMut(LookSet) -> LookSet) {}
    pub(crate) fn set_look_need(&mut self, set: impl FnMut(LookSet) -> LookSet) {}
    pub(crate) fn add_nfa_state_id(&mut self, sid: StateID) {
        ReprVec(&mut self.repr).add_nfa_state_id(&mut self.prev_nfa_state_id, sid)
    }
    pub(crate) fn as_bytes(&self) -> &[u8] {}
    fn repr(&self) -> Repr<'_> {}
    fn repr_vec(&mut self) -> ReprVec<'_> {}
}
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
    fn close_match_pattern_ids(&mut self) {}
    fn add_nfa_state_id(&mut self, prev: &mut StateID, sid: StateID) {
        let delta = sid.as_i32() - prev.as_i32();
        write_vari32(self.0, delta);
        *prev = sid;
    }
    fn repr(&self) -> Repr<'_> {}
}
