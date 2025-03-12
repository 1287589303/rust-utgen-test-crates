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
struct Epsilons(u64);
#[derive(Clone, Copy)]
struct Slots(u32);
impl Epsilons {
    const SLOT_MASK: u64 = 0x000003FF_FFFFFC00;
    const SLOT_SHIFT: u64 = 10;
    const LOOK_MASK: u64 = 0x00000000_000003FF;
    fn empty() -> Epsilons {}
    fn is_empty(self) -> bool {}
    fn slots(self) -> Slots {}
    fn set_slots(self, slots: Slots) -> Epsilons {
        Epsilons(
            (u64::from(slots.0) << Epsilons::SLOT_SHIFT) | (self.0 & Epsilons::LOOK_MASK),
        )
    }
    fn looks(self) -> LookSet {}
    fn set_looks(self, look_set: LookSet) -> Epsilons {}
}
