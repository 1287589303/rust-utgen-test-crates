use alloc::vec::Vec;
use crate::util::primitives::StateID;
#[derive(Debug)]
pub(super) struct Remapper {
    /// A map from the index of a state to its pre-multiplied identifier.
    ///
    /// When a state is swapped with another, then their corresponding
    /// locations in this map are also swapped. Thus, its new position will
    /// still point to its old pre-multiplied StateID.
    ///
    /// While there is a bit more to it, this then allows us to rewrite the
    /// state IDs in a DFA's transition table in a single pass. This is done
    /// by iterating over every ID in this map, then iterating over each
    /// transition for the state at that ID and re-mapping the transition from
    /// `old_id` to `map[dfa.to_index(old_id)]`. That is, we find the position
    /// in this map where `old_id` *started*, and set it to where it ended up
    /// after all swaps have been completed.
    map: Vec<StateID>,
    /// A mapper from state index to state ID (and back).
    idxmap: IndexMapper,
}
#[derive(Debug)]
struct IndexMapper {
    /// The power of 2 corresponding to the stride of the corresponding
    /// transition table. 'id >> stride2' de-multiplies an ID while 'index <<
    /// stride2' pre-multiplies an index to an ID.
    stride2: usize,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
impl Remapper {
    pub(super) fn new(r: &impl Remappable) -> Remapper {}
    pub(super) fn swap(&mut self, r: &mut impl Remappable, id1: StateID, id2: StateID) {
        if id1 == id2 {
            return;
        }
        r.swap_states(id1, id2);
        self.map.swap(self.idxmap.to_index(id1), self.idxmap.to_index(id2));
    }
    pub(super) fn remap(mut self, r: &mut impl Remappable) {}
}
impl IndexMapper {
    fn to_index(&self, id: StateID) -> usize {
        id.as_usize() >> self.stride2
    }
    fn to_state_id(&self, index: usize) -> StateID {}
}
