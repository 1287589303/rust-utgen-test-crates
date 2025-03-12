use alloc::{vec, vec::Vec};
use crate::{
    int::{NonMaxUsize, U32},
    nfa::{State, StateID, NFA},
    pool::CachePoolGuard, utf8,
};
#[derive(Clone)]
struct SparseSet {
    /// The number of elements currently in this set.
    len: usize,
    /// Dense contains the ids in the order in which they were inserted.
    dense: Vec<StateID>,
    /// Sparse maps ids to their location in dense.
    ///
    /// A state ID is in the set if and only if
    /// sparse[id] < len && id == dense[sparse[id]].
    ///
    /// Note that these are indices into 'dense'. It's a little weird to use
    /// StateID here, but we know our length can never exceed the bounds of
    /// StateID (enforced by 'resize') and StateID will be at most 4 bytes
    /// where as a usize is likely double that in most cases.
    sparse: Vec<StateID>,
}
impl SparseSet {
    fn new(capacity: usize) -> SparseSet {}
    fn resize(&mut self, new_capacity: usize) {}
    fn capacity(&self) -> usize {
        self.dense.len()
    }
    fn len(&self) -> usize {}
    fn is_empty(&self) -> bool {}
    fn insert(&mut self, id: StateID) -> bool {}
    fn contains(&self, id: StateID) -> bool {}
    fn clear(&mut self) {}
    fn iter(&self) -> SparseSetIter<'_> {}
}
