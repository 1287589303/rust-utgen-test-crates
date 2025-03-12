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
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
struct Repr<'a>(&'a [u8]);
impl core::fmt::Debug for StateBuilderNFA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_tuple("StateBuilderNFA").field(&self.repr()).finish()
    }
}
impl StateBuilderNFA {
    pub(crate) fn to_state(&self) -> State {}
    pub(crate) fn clear(self) -> StateBuilderEmpty {}
    pub(crate) fn look_need(&self) -> LookSet {}
    pub(crate) fn set_look_have(&mut self, set: impl FnMut(LookSet) -> LookSet) {}
    pub(crate) fn set_look_need(&mut self, set: impl FnMut(LookSet) -> LookSet) {}
    pub(crate) fn add_nfa_state_id(&mut self, sid: StateID) {}
    pub(crate) fn as_bytes(&self) -> &[u8] {}
    fn repr(&self) -> Repr<'_> {
        Repr(&self.repr)
    }
    fn repr_vec(&mut self) -> ReprVec<'_> {}
}
