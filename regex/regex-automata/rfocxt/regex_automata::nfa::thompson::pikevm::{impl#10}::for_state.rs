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
struct SlotTable {
    /// The actual table of offsets.
    table: Vec<Option<NonMaxUsize>>,
    /// The number of slots per state, i.e., the table's stride or the length
    /// of each row.
    slots_per_state: usize,
    /// The number of slots in the caller-provided 'Captures' value for the
    /// current search. Setting this to 'slots_per_state' is always correct,
    /// but may be wasteful.
    slots_for_captures: usize,
}
#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct NonMaxUsize(NonZeroUsize);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
impl SlotTable {
    fn new() -> SlotTable {}
    fn reset(&mut self, re: &PikeVM) {}
    fn memory_usage(&self) -> usize {}
    fn setup_search(&mut self, captures_slot_len: usize) {}
    fn for_state(&mut self, sid: StateID) -> &mut [Option<NonMaxUsize>] {
        let i = sid.as_usize() * self.slots_per_state;
        &mut self.table[i..i + self.slots_for_captures]
    }
    fn all_absent(&mut self) -> &mut [Option<NonMaxUsize>] {}
}
