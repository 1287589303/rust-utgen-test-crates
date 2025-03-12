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
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
impl StateBuilderMatches {
    pub(crate) fn into_nfa(mut self) -> StateBuilderNFA {}
    pub(crate) fn set_is_from_word(&mut self) {}
    pub(crate) fn set_is_half_crlf(&mut self) {}
    pub(crate) fn look_have(&self) -> LookSet {}
    pub(crate) fn set_look_have(&mut self, set: impl FnMut(LookSet) -> LookSet) {}
    pub(crate) fn add_match_pattern_id(&mut self, pid: PatternID) {
        self.repr_vec().add_match_pattern_id(pid)
    }
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
    fn add_match_pattern_id(&mut self, pid: PatternID) {
        if !self.repr().has_pattern_ids() {
            if pid == PatternID::ZERO {
                self.set_is_match();
                return;
            }
            self.0.extend(core::iter::repeat(0).take(PatternID::SIZE));
            self.set_has_pattern_ids();
            if self.repr().is_match() {
                write_u32(self.0, 0)
            } else {
                self.set_is_match();
            }
        }
        write_u32(self.0, pid.as_u32());
    }
    fn close_match_pattern_ids(&mut self) {}
    fn add_nfa_state_id(&mut self, prev: &mut StateID, sid: StateID) {}
    fn repr(&self) -> Repr<'_> {}
}
