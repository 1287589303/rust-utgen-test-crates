#[cfg(feature = "internal-instrument-pikevm")]
use core::cell::RefCell;
use alloc::{vec, vec::Vec};
use crate::{
    nfa::thompson::{self, BuildError, State, NFA},
    util::{
        captures::Captures, empty, iter, prefilter::Prefilter,
        primitives::{NonMaxUsize, PatternID, SmallIndex, StateID},
        search::{Anchored, HalfMatch, Input, Match, MatchKind, PatternSet, Span},
        sparse_set::SparseSet,
    },
};
#[derive(Clone, Debug)]
pub struct Cache {
    /// Stack used while computing epsilon closure. This effectively lets us
    /// move what is more naturally expressed through recursion to a stack
    /// on the heap.
    stack: Vec<FollowEpsilon>,
    /// The current active states being explored for the current byte in the
    /// haystack.
    curr: ActiveStates,
    /// The next set of states we're building that will be explored for the
    /// next byte in the haystack.
    next: ActiveStates,
}
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
enum FollowEpsilon {
    /// Explore the epsilon transitions from a state ID.
    Explore(StateID),
    /// Reset the given `slot` to the given `offset` (which might be `None`).
    RestoreCapture { slot: SmallIndex, offset: Option<NonMaxUsize> },
}
impl Cache {
    pub fn new(re: &PikeVM) -> Cache {}
    pub fn reset(&mut self, re: &PikeVM) {}
    pub fn memory_usage(&self) -> usize {
        use core::mem::size_of;
        (self.stack.len() * size_of::<FollowEpsilon>()) + self.curr.memory_usage()
            + self.next.memory_usage()
    }
    fn setup_search(&mut self, captures_slot_len: usize) {}
}
impl ActiveStates {
    fn new(re: &PikeVM) -> ActiveStates {}
    fn reset(&mut self, re: &PikeVM) {}
    fn memory_usage(&self) -> usize {
        self.set.memory_usage() + self.slot_table.memory_usage()
    }
    fn setup_search(&mut self, captures_slot_len: usize) {}
}
