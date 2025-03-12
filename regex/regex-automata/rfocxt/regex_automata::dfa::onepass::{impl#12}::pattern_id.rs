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
pub(crate) trait U64 {
    fn as_usize(self) -> usize;
    fn low_u8(self) -> u8;
    fn low_u16(self) -> u16;
    fn low_u32(self) -> u32;
    fn high_u32(self) -> u32;
}
#[derive(Clone, Copy)]
struct PatternEpsilons(u64);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
impl PatternEpsilons {
    const PATTERN_ID_BITS: u64 = 22;
    const PATTERN_ID_SHIFT: u64 = 64 - PatternEpsilons::PATTERN_ID_BITS;
    const PATTERN_ID_NONE: u64 = 0x00000000_003FFFFF;
    const PATTERN_ID_LIMIT: u64 = PatternEpsilons::PATTERN_ID_NONE;
    const PATTERN_ID_MASK: u64 = 0xFFFFFC00_00000000;
    const EPSILONS_MASK: u64 = 0x000003FF_FFFFFFFF;
    fn empty() -> PatternEpsilons {}
    fn is_empty(self) -> bool {}
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
    fn epsilons(self) -> Epsilons {}
    fn set_epsilons(self, epsilons: Epsilons) -> PatternEpsilons {}
}
