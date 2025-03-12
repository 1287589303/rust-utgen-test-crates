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
#[derive(Clone, Debug)]
pub(crate) struct StateBuilderEmpty(Vec<u8>);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
impl StateBuilderNFA {
    pub(crate) fn to_state(&self) -> State {}
    pub(crate) fn clear(self) -> StateBuilderEmpty {
        let mut builder = StateBuilderEmpty(self.repr);
        builder.clear();
        builder
    }
    pub(crate) fn look_need(&self) -> LookSet {}
    pub(crate) fn set_look_have(&mut self, set: impl FnMut(LookSet) -> LookSet) {}
    pub(crate) fn set_look_need(&mut self, set: impl FnMut(LookSet) -> LookSet) {}
    pub(crate) fn add_nfa_state_id(&mut self, sid: StateID) {}
    pub(crate) fn as_bytes(&self) -> &[u8] {}
    fn repr(&self) -> Repr<'_> {}
    fn repr_vec(&mut self) -> ReprVec<'_> {}
}
impl StateBuilderEmpty {
    pub(crate) fn new() -> StateBuilderEmpty {}
    pub(crate) fn into_matches(mut self) -> StateBuilderMatches {}
    fn clear(&mut self) {
        self.0.clear();
    }
    pub(crate) fn capacity(&self) -> usize {}
}
