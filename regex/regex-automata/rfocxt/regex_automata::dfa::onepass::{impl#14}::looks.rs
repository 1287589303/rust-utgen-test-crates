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
struct Epsilons(u64);
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct LookSet {
    /// The underlying representation this set is exposed to make it possible
    /// to store it somewhere efficiently. The representation is that
    /// of a bitset, where each assertion occupies bit `i` where
    /// `i = Look::as_repr()`.
    ///
    /// Note that users of this internal representation must permit the full
    /// range of `u16` values to be represented. For example, even if the
    /// current implementation only makes use of the 10 least significant bits,
    /// it may use more bits in a future semver compatible release.
    pub bits: u32,
}
impl Epsilons {
    const SLOT_MASK: u64 = 0x000003FF_FFFFFC00;
    const SLOT_SHIFT: u64 = 10;
    const LOOK_MASK: u64 = 0x00000000_000003FF;
    fn empty() -> Epsilons {}
    fn is_empty(self) -> bool {}
    fn slots(self) -> Slots {}
    fn set_slots(self, slots: Slots) -> Epsilons {}
    fn looks(self) -> LookSet {
        LookSet {
            bits: (self.0 & Epsilons::LOOK_MASK).low_u32(),
        }
    }
    fn set_looks(self, look_set: LookSet) -> Epsilons {}
}
