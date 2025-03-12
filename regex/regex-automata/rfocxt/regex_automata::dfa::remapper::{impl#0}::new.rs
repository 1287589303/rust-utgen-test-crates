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
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
#[derive(Debug)]
struct IndexMapper {
    /// The power of 2 corresponding to the stride of the corresponding
    /// transition table. 'id >> stride2' de-multiplies an ID while 'index <<
    /// stride2' pre-multiplies an index to an ID.
    stride2: usize,
}
impl Remapper {
    pub(super) fn new(r: &impl Remappable) -> Remapper {
        let idxmap = IndexMapper {
            stride2: r.stride2(),
        };
        let map = (0..r.state_len()).map(|i| idxmap.to_state_id(i)).collect();
        Remapper { map, idxmap }
    }
    pub(super) fn swap(&mut self, r: &mut impl Remappable, id1: StateID, id2: StateID) {}
    pub(super) fn remap(mut self, r: &mut impl Remappable) {}
}
