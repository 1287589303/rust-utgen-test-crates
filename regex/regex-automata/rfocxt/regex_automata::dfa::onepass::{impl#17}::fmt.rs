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
#[derive(Debug)]
struct SlotsIter {
    slots: Slots,
}
impl core::fmt::Debug for Slots {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "S")?;
        for slot in self.iter() {
            write!(f, "-{:?}", slot)?;
        }
        Ok(())
    }
}
impl Iterator for SlotsIter {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let slot = self.slots.0.trailing_zeros().as_usize();
        if slot >= Slots::LIMIT {
            return None;
        }
        self.slots = self.slots.remove(slot);
        Some(slot)
    }
}
impl Slots {
    const LIMIT: usize = 32;
    fn insert(self, slot: usize) -> Slots {}
    fn remove(self, slot: usize) -> Slots {}
    fn is_empty(self) -> bool {}
    fn iter(self) -> SlotsIter {
        SlotsIter { slots: self }
    }
    fn apply(self, at: usize, caller_explicit_slots: &mut [Option<NonMaxUsize>]) {}
}
