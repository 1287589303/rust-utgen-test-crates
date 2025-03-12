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
pub(crate) trait Usize {
    fn as_u8(self) -> u8;
    fn as_u16(self) -> u16;
    fn as_u32(self) -> u32;
    fn as_u64(self) -> u64;
}
#[derive(Clone, Copy)]
struct Slots(u32);
impl Slots {
    const LIMIT: usize = 32;
    fn insert(self, slot: usize) -> Slots {}
    fn remove(self, slot: usize) -> Slots {
        debug_assert!(slot < Slots::LIMIT);
        Slots(self.0 & !(1 << slot.as_u32()))
    }
    fn is_empty(self) -> bool {}
    fn iter(self) -> SlotsIter {}
    fn apply(self, at: usize, caller_explicit_slots: &mut [Option<NonMaxUsize>]) {}
}
