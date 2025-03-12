use alloc::{vec, vec::Vec};
use crate::util::primitives::StateID;
#[derive(Clone, Debug)]
pub(crate) struct SparseSets {
    pub(crate) set1: SparseSet,
    pub(crate) set2: SparseSet,
}
#[derive(Clone)]
pub(crate) struct SparseSet {
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
impl SparseSets {
    pub(crate) fn new(capacity: usize) -> SparseSets {
        SparseSets {
            set1: SparseSet::new(capacity),
            set2: SparseSet::new(capacity),
        }
    }
    #[inline]
    pub(crate) fn resize(&mut self, new_capacity: usize) {}
    pub(crate) fn clear(&mut self) {}
    pub(crate) fn swap(&mut self) {}
    pub(crate) fn memory_usage(&self) -> usize {}
}
impl SparseSet {
    #[inline]
    pub(crate) fn new(capacity: usize) -> SparseSet {
        let mut set = SparseSet {
            len: 0,
            dense: vec![],
            sparse: vec![],
        };
        set.resize(capacity);
        set
    }
    #[inline]
    pub(crate) fn resize(&mut self, new_capacity: usize) {}
    #[inline]
    pub(crate) fn capacity(&self) -> usize {}
    #[inline]
    pub(crate) fn len(&self) -> usize {}
    #[inline]
    pub(crate) fn is_empty(&self) -> bool {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn insert(&mut self, id: StateID) -> bool {}
    #[inline]
    pub(crate) fn contains(&self, id: StateID) -> bool {}
    #[inline]
    pub(crate) fn clear(&mut self) {}
    #[inline]
    pub(crate) fn iter(&self) -> SparseSetIter<'_> {}
    #[inline]
    pub(crate) fn memory_usage(&self) -> usize {}
}
