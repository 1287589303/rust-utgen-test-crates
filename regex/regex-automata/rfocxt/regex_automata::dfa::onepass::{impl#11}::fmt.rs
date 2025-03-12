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
#[derive(Clone, Copy, Eq, PartialEq)]
struct Transition(u64);
#[derive(Clone, Copy)]
struct Epsilons(u64);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
impl core::fmt::Debug for Transition {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        if self.is_dead() {
            return write!(f, "0");
        }
        write!(f, "{}", self.state_id().as_usize())?;
        if self.match_wins() {
            write!(f, "-MW")?;
        }
        if !self.epsilons().is_empty() {
            write!(f, "-{:?}", self.epsilons())?;
        }
        Ok(())
    }
}
impl Transition {
    const STATE_ID_BITS: u64 = 21;
    const STATE_ID_SHIFT: u64 = 64 - Transition::STATE_ID_BITS;
    const STATE_ID_LIMIT: u64 = 1 << Transition::STATE_ID_BITS;
    const MATCH_WINS_SHIFT: u64 = 64 - (Transition::STATE_ID_BITS + 1);
    const INFO_MASK: u64 = 0x000003FF_FFFFFFFF;
    fn new(match_wins: bool, sid: StateID, epsilons: Epsilons) -> Transition {}
    fn is_dead(self) -> bool {
        self.state_id() == DEAD
    }
    fn match_wins(&self) -> bool {
        (self.0 >> Transition::MATCH_WINS_SHIFT & 1) == 1
    }
    fn state_id(&self) -> StateID {
        StateID::new_unchecked((self.0 >> Transition::STATE_ID_SHIFT).as_usize())
    }
    fn set_state_id(&mut self, sid: StateID) {}
    fn epsilons(&self) -> Epsilons {
        Epsilons(self.0 & Transition::INFO_MASK)
    }
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
