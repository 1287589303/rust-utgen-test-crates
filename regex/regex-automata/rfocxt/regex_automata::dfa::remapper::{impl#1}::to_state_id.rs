use alloc::vec::Vec;
use crate::util::primitives::StateID;
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
impl IndexMapper {
    fn to_index(&self, id: StateID) -> usize {}
    fn to_state_id(&self, index: usize) -> StateID {
        StateID::new_unchecked(index << self.stride2)
    }
}
