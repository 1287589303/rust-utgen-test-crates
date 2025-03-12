use alloc::{vec, vec::Vec};
use crate::{
    int::{NonMaxUsize, U32},
    nfa::{State, StateID, NFA},
    pool::CachePoolGuard, utf8,
};
#[derive(Clone, Debug)]
pub(crate) struct PikeVM {
    nfa: NFA,
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
pub(crate) struct NFA {
    /// The pattern string this NFA was generated from.
    ///
    /// We put it here for lack of a better place to put it. ¯\_(ツ)_/¯
    pattern: String,
    /// The states that make up this NFA.
    states: Vec<State>,
    /// The ID of the start state.
    start: StateID,
    /// Whether this NFA can only match at the beginning of a haystack.
    is_start_anchored: bool,
    /// Whether this NFA can match the empty string.
    is_match_empty: bool,
    /// If every match has the same number of matching capture groups, then
    /// this corresponds to the number of groups.
    static_explicit_captures_len: Option<usize>,
    /// A map from capture group name to its corresponding index.
    cap_name_to_index: CaptureNameMap,
    /// A map from capture group index to the corresponding name, if one
    /// exists.
    cap_index_to_name: Vec<Option<Arc<str>>>,
    /// Heap memory used indirectly by NFA states and other things (like the
    /// various capturing group representations above). Since each state
    /// might use a different amount of heap, we need to keep track of this
    /// incrementally.
    memory_extra: usize,
}
#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub(crate) struct NonMaxUsize(NonZeroUsize);
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
#[derive(Debug)]
struct SparseSetIter<'a>(core::slice::Iter<'a, StateID>);
#[derive(Clone, Debug)]
enum FollowEpsilon {
    /// Explore the epsilon transitions from a state ID.
    Explore(StateID),
    /// Reset the given `slot` to the given `offset` (which might be `None`).
    RestoreCapture { slot: u32, offset: Option<NonMaxUsize> },
}
impl PikeVM {
    pub(crate) fn new(nfa: NFA) -> PikeVM {}
    pub(crate) fn nfa(&self) -> &NFA {}
    pub(crate) fn find_iter<'r, 'h>(
        &'r self,
        cache: CachePoolGuard<'r>,
        haystack: &'h [u8],
    ) -> FindMatches<'r, 'h> {}
    pub(crate) fn captures_iter<'r, 'h>(
        &'r self,
        cache: CachePoolGuard<'r>,
        haystack: &'h [u8],
    ) -> CapturesMatches<'r, 'h> {}
    pub(crate) fn search(
        &self,
        cache: &mut Cache,
        haystack: &[u8],
        start: usize,
        end: usize,
        earliest: bool,
        slots: &mut [Option<NonMaxUsize>],
    ) -> bool {}
    fn nexts(
        &self,
        stack: &mut Vec<FollowEpsilon>,
        curr: &mut ActiveStates,
        next: &mut ActiveStates,
        haystack: &[u8],
        at: usize,
        at_ch: char,
        at_len: usize,
        slots: &mut [Option<NonMaxUsize>],
    ) -> bool {
        let ActiveStates { ref set, ref mut slot_table } = *curr;
        for sid in set.iter() {
            if self.next(stack, slot_table, next, haystack, at, at_ch, at_len, sid) {
                slots.copy_from_slice(slot_table.for_state(sid));
                return true;
            }
        }
        false
    }
    fn next(
        &self,
        stack: &mut Vec<FollowEpsilon>,
        curr_slot_table: &mut SlotTable,
        next: &mut ActiveStates,
        haystack: &[u8],
        at: usize,
        at_ch: char,
        at_len: usize,
        sid: StateID,
    ) -> bool {
        match *self.nfa.state(sid) {
            State::Fail
            | State::Goto { .. }
            | State::Splits { .. }
            | State::Capture { .. } => false,
            State::Char { target, ch } => {
                if at_ch == ch && at_len > 0 {
                    let slots = curr_slot_table.for_state(sid);
                    let at = at.wrapping_add(at_len);
                    self.epsilon_closure(stack, slots, next, haystack, at, target);
                }
                false
            }
            State::Ranges { target, ref ranges } => {
                for (start, end) in ranges.iter().copied() {
                    if start > at_ch {
                        break;
                    } else if start <= at_ch && at_ch <= end {
                        if at_len == 0 {
                            return false;
                        }
                        let slots = curr_slot_table.for_state(sid);
                        let at = at.wrapping_add(at_len);
                        self.epsilon_closure(stack, slots, next, haystack, at, target);
                    }
                }
                false
            }
            State::Match => true,
        }
    }
    fn epsilon_closure(
        &self,
        stack: &mut Vec<FollowEpsilon>,
        curr_slots: &mut [Option<NonMaxUsize>],
        next: &mut ActiveStates,
        haystack: &[u8],
        at: usize,
        sid: StateID,
    ) {}
    fn epsilon_closure_explore(
        &self,
        stack: &mut Vec<FollowEpsilon>,
        curr_slots: &mut [Option<NonMaxUsize>],
        next: &mut ActiveStates,
        haystack: &[u8],
        at: usize,
        mut sid: StateID,
    ) {}
}
impl SparseSet {
    fn new(capacity: usize) -> SparseSet {}
    fn resize(&mut self, new_capacity: usize) {}
    fn capacity(&self) -> usize {}
    fn len(&self) -> usize {}
    fn is_empty(&self) -> bool {}
    fn insert(&mut self, id: StateID) -> bool {}
    fn contains(&self, id: StateID) -> bool {}
    fn clear(&mut self) {}
    fn iter(&self) -> SparseSetIter<'_> {
        SparseSetIter(self.dense[..self.len()].iter())
    }
}
impl SlotTable {
    fn new() -> SlotTable {}
    fn reset(&mut self, re: &PikeVM) {}
    fn setup_search(&mut self, captures_slot_len: usize) {}
    fn for_state(&mut self, sid: StateID) -> &mut [Option<NonMaxUsize>] {
        let i = sid.as_usize() * self.slots_per_state;
        &mut self.table[i..i + self.slots_for_captures]
    }
    fn all_absent(&mut self) -> &mut [Option<NonMaxUsize>] {}
}
