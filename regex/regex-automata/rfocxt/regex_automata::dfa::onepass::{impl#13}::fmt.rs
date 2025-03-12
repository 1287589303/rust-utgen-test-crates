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
struct PatternEpsilons(u64);
#[derive(Clone, Copy)]
struct Epsilons(u64);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
impl core::fmt::Debug for PatternEpsilons {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if self.is_empty() {
            return write!(f, "N/A");
        }
        if let Some(pid) = self.pattern_id() {
            write!(f, "{}", pid.as_usize())?;
        }
        if !self.epsilons().is_empty() {
            if self.pattern_id().is_some() {
                write!(f, "/")?;
            }
            write!(f, "{:?}", self.epsilons())?;
        }
        Ok(())
    }
}
impl PatternEpsilons {
    const PATTERN_ID_BITS: u64 = 22;
    const PATTERN_ID_SHIFT: u64 = 64 - PatternEpsilons::PATTERN_ID_BITS;
    const PATTERN_ID_NONE: u64 = 0x00000000_003FFFFF;
    const PATTERN_ID_LIMIT: u64 = PatternEpsilons::PATTERN_ID_NONE;
    const PATTERN_ID_MASK: u64 = 0xFFFFFC00_00000000;
    const EPSILONS_MASK: u64 = 0x000003FF_FFFFFFFF;
    fn empty() -> PatternEpsilons {}
    fn is_empty(self) -> bool {
        self.pattern_id().is_none() && self.epsilons().is_empty()
    }
    fn pattern_id(self) -> Option<PatternID> {
        let pid = self.0 >> PatternEpsilons::PATTERN_ID_SHIFT;
        if pid == PatternEpsilons::PATTERN_ID_LIMIT {
            None
        } else {
            Some(PatternID::new_unchecked(pid.as_usize()))
        }
    }
    fn pattern_id_unchecked(self) -> PatternID {}
    fn set_pattern_id(self, pid: PatternID) -> PatternEpsilons {}
    fn epsilons(self) -> Epsilons {
        Epsilons(self.0 & PatternEpsilons::EPSILONS_MASK)
    }
    fn set_epsilons(self, epsilons: Epsilons) -> PatternEpsilons {}
}
impl Epsilons {
    const SLOT_MASK: u64 = 0x000003FF_FFFFFC00;
    const SLOT_SHIFT: u64 = 10;
    const LOOK_MASK: u64 = 0x00000000_000003FF;
    fn empty() -> Epsilons {}
    fn is_empty(self) -> bool {
        self.0 == 0
    }
    fn slots(self) -> Slots {}
    fn set_slots(self, slots: Slots) -> Epsilons {}
    fn looks(self) -> LookSet {}
    fn set_looks(self, look_set: LookSet) -> Epsilons {}
}
