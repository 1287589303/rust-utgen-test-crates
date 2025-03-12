use alloc::{vec, vec::Vec};
use crate::{
    int::{NonMaxUsize, U32},
    nfa::{State, StateID, NFA},
    pool::CachePoolGuard, utf8,
};
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
#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub(crate) struct NonMaxUsize(NonZeroUsize);
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
