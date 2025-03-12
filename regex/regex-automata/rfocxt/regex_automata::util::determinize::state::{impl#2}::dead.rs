use core::mem;
use alloc::{sync::Arc, vec::Vec};
use crate::util::{
    int::{I32, U32},
    look::LookSet, primitives::{PatternID, StateID},
    wire::{self, Endian},
};
#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub(crate) struct State(Arc<[u8]>);
#[derive(Clone, Debug)]
pub(crate) struct StateBuilderEmpty(Vec<u8>);
#[derive(Clone)]
pub(crate) struct StateBuilderMatches(Vec<u8>);
#[derive(Clone)]
pub(crate) struct StateBuilderNFA {
    repr: Vec<u8>,
    prev_nfa_state_id: StateID,
}
impl State {
    pub(crate) fn dead() -> State {
        StateBuilderEmpty::new().into_matches().into_nfa().to_state()
    }
    pub(crate) fn is_match(&self) -> bool {}
    pub(crate) fn is_from_word(&self) -> bool {}
    pub(crate) fn is_half_crlf(&self) -> bool {}
    pub(crate) fn look_have(&self) -> LookSet {}
    pub(crate) fn look_need(&self) -> LookSet {}
    pub(crate) fn match_len(&self) -> usize {}
    pub(crate) fn match_pattern(&self, index: usize) -> PatternID {}
    pub(crate) fn match_pattern_ids(&self) -> Option<Vec<PatternID>> {}
    #[cfg(all(test, not(miri)))]
    pub(crate) fn iter_match_pattern_ids<F: FnMut(PatternID)>(&self, f: F) {}
    pub(crate) fn iter_nfa_state_ids<F: FnMut(StateID)>(&self, f: F) {}
    pub(crate) fn memory_usage(&self) -> usize {}
    fn repr(&self) -> Repr<'_> {}
}
impl StateBuilderEmpty {
    pub(crate) fn new() -> StateBuilderEmpty {
        StateBuilderEmpty(alloc::vec![])
    }
    pub(crate) fn into_matches(mut self) -> StateBuilderMatches {
        self.0.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0]);
        StateBuilderMatches(self.0)
    }
    fn clear(&mut self) {}
    pub(crate) fn capacity(&self) -> usize {}
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
    fn repr_vec(&mut self) -> ReprVec<'_> {}
}
impl StateBuilderNFA {
    pub(crate) fn to_state(&self) -> State {
        State(Arc::from(&*self.repr))
    }
    pub(crate) fn clear(self) -> StateBuilderEmpty {}
    pub(crate) fn look_need(&self) -> LookSet {}
    pub(crate) fn set_look_have(&mut self, set: impl FnMut(LookSet) -> LookSet) {}
    pub(crate) fn set_look_need(&mut self, set: impl FnMut(LookSet) -> LookSet) {}
    pub(crate) fn add_nfa_state_id(&mut self, sid: StateID) {}
    pub(crate) fn as_bytes(&self) -> &[u8] {}
    fn repr(&self) -> Repr<'_> {}
    fn repr_vec(&mut self) -> ReprVec<'_> {}
}
