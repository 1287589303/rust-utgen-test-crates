use crate::{dfa::dense::OwnedDFA, util::primitives::StateID};
use super::Remappable;
pub(super) trait Remappable: core::fmt::Debug {
    fn state_len(&self) -> usize;
    fn stride2(&self) -> usize;
    fn swap_states(&mut self, id1: StateID, id2: StateID);
    fn remap(&mut self, map: impl Fn(StateID) -> StateID);
}
impl Remappable for OwnedDFA {
    fn state_len(&self) -> usize {}
    fn stride2(&self) -> usize {
        OwnedDFA::stride2(self)
    }
    fn swap_states(&mut self, id1: StateID, id2: StateID) {}
    fn remap(&mut self, map: impl Fn(StateID) -> StateID) {}
}
