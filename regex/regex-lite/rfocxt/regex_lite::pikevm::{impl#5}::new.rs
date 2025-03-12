use alloc::{vec, vec::Vec};
use crate::{
    int::{NonMaxUsize, U32},
    nfa::{State, StateID, NFA},
    pool::CachePoolGuard, utf8,
};
#[derive(Clone, Debug)]
struct ActiveStates {
    /// The set of active NFA states. This set preserves insertion order, which
    /// is critical for simulating the match semantics of backtracking regex
    /// engines.
    set: SparseSet,
    /// The slots for every NFA state, where each slot stores a (possibly
    /// absent) offset. Every capturing group has two slots. One for a start
    /// offset and one for an end offset.
    slot_table: SlotTable,
}
#[derive(Clone, Debug)]
struct SlotTable {
    /// The actual table of offsets.
    table: Vec<Option<NonMaxUsize>>,
    /// The number of slots per state, i.e., the table's stride or the length
    /// of each row.
    slots_per_state: usize,
    /// The number of slots in the caller-provided `Captures` value for the
    /// current search. Setting this to `slots_per_state` is always correct,
    /// but may be wasteful.
    slots_for_captures: usize,
}
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
#[derive(Clone, Debug)]
pub(crate) struct PikeVM {
    nfa: NFA,
}
impl ActiveStates {
    fn new(re: &PikeVM) -> ActiveStates {
        let mut active = ActiveStates {
            set: SparseSet::new(0),
            slot_table: SlotTable::new(),
        };
        active.reset(re);
        active
    }
    fn reset(&mut self, re: &PikeVM) {
        self.set.resize(re.nfa().len());
        self.slot_table.reset(re);
    }
    fn setup_search(&mut self, captures_slot_len: usize) {}
}
impl SlotTable {
    fn new() -> SlotTable {
        SlotTable {
            table: vec![],
            slots_for_captures: 0,
            slots_per_state: 0,
        }
    }
    fn reset(&mut self, re: &PikeVM) {}
    fn setup_search(&mut self, captures_slot_len: usize) {}
    fn for_state(&mut self, sid: StateID) -> &mut [Option<NonMaxUsize>] {}
    fn all_absent(&mut self) -> &mut [Option<NonMaxUsize>] {}
}
impl SparseSet {
    fn new(capacity: usize) -> SparseSet {
        let mut set = SparseSet {
            len: 0,
            dense: vec![],
            sparse: vec![],
        };
        set.resize(capacity);
        set
    }
    fn resize(&mut self, new_capacity: usize) {}
    fn capacity(&self) -> usize {}
    fn len(&self) -> usize {}
    fn is_empty(&self) -> bool {}
    fn insert(&mut self, id: StateID) -> bool {}
    fn contains(&self, id: StateID) -> bool {}
    fn clear(&mut self) {}
    fn iter(&self) -> SparseSetIter<'_> {}
}
