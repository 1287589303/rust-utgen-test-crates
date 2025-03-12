use alloc::{vec, vec::Vec};
use crate::{
    dfa::{remapper::Remapper, DEAD},
    nfa::thompson::{self, NFA},
    util::{
        alphabet::ByteClasses, captures::Captures, escape::DebugByte,
        int::{Usize, U32, U64, U8},
        look::{Look, LookSet, UnicodeWordBoundaryError},
        primitives::{NonMaxUsize, PatternID, StateID},
        search::{Anchored, Input, Match, MatchError, MatchKind, Span},
        sparse_set::SparseSet,
    },
};
#[derive(Clone, Copy)]
struct Slots(u32);
impl Slots {
    const LIMIT: usize = 32;
    fn insert(self, slot: usize) -> Slots {}
    fn remove(self, slot: usize) -> Slots {}
    fn is_empty(self) -> bool {
        self.0 == 0
    }
    fn iter(self) -> SlotsIter {}
    fn apply(self, at: usize, caller_explicit_slots: &mut [Option<NonMaxUsize>]) {}
}
