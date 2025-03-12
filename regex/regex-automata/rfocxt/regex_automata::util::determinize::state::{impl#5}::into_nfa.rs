use core::mem;
use alloc::{sync::Arc, vec::Vec};
use crate::util::{
    int::{I32, U32},
    look::LookSet, primitives::{PatternID, StateID},
    wire::{self, Endian},
};
#[derive(Clone)]
pub(crate) struct StateBuilderMatches(Vec<u8>);
struct ReprVec<'a>(&'a mut Vec<u8>);
#[derive(Clone)]
pub(crate) struct StateBuilderNFA {
    repr: Vec<u8>,
    prev_nfa_state_id: StateID,
}
impl StateBuilderMatches {
    pub(crate) fn into_nfa(mut self) -> StateBuilderNFA {
        self.repr_vec().close_match_pattern_ids();
        StateBuilderNFA {
            repr: self.0,
            prev_nfa_state_id: StateID::ZERO,
        }
    }
    pub(crate) fn set_is_from_word(&mut self) {}
    pub(crate) fn set_is_half_crlf(&mut self) {}
    pub(crate) fn look_have(&self) -> LookSet {}
    pub(crate) fn set_look_have(&mut self, set: impl FnMut(LookSet) -> LookSet) {}
    pub(crate) fn add_match_pattern_id(&mut self, pid: PatternID) {}
    fn repr(&self) -> Repr<'_> {}
    fn repr_vec(&mut self) -> ReprVec<'_> {
        ReprVec(&mut self.0)
    }
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
    fn repr(&self) -> Repr<'_> {}
}
