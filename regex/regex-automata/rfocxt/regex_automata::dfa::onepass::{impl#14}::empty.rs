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
impl Epsilons {
    const SLOT_MASK: u64 = 0x000003FF_FFFFFC00;
    const SLOT_SHIFT: u64 = 10;
    const LOOK_MASK: u64 = 0x00000000_000003FF;
    fn empty() -> Epsilons {
        Epsilons(0)
    }
    fn is_empty(self) -> bool {}
    fn slots(self) -> Slots {}
    fn set_slots(self, slots: Slots) -> Epsilons {}
    fn looks(self) -> LookSet {}
    fn set_looks(self, look_set: LookSet) -> Epsilons {}
}
