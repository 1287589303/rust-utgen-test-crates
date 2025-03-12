use core::mem;
use alloc::{sync::Arc, vec::Vec};
use crate::util::{
    int::{I32, U32},
    look::LookSet, primitives::{PatternID, StateID},
    wire::{self, Endian},
};
#[derive(Clone)]
pub(crate) struct StateBuilderMatches(Vec<u8>);
struct Repr<'a>(&'a [u8]);
impl StateBuilderMatches {
    pub(crate) fn into_nfa(mut self) -> StateBuilderNFA {}
    pub(crate) fn set_is_from_word(&mut self) {}
    pub(crate) fn set_is_half_crlf(&mut self) {}
    pub(crate) fn look_have(&self) -> LookSet {}
    pub(crate) fn set_look_have(&mut self, set: impl FnMut(LookSet) -> LookSet) {}
    pub(crate) fn add_match_pattern_id(&mut self, pid: PatternID) {}
    fn repr(&self) -> Repr<'_> {
        Repr(&self.0)
    }
    fn repr_vec(&mut self) -> ReprVec<'_> {}
}
