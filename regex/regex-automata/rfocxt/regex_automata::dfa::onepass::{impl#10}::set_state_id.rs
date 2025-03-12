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
impl Transition {
    const STATE_ID_BITS: u64 = 21;
    const STATE_ID_SHIFT: u64 = 64 - Transition::STATE_ID_BITS;
    const STATE_ID_LIMIT: u64 = 1 << Transition::STATE_ID_BITS;
    const MATCH_WINS_SHIFT: u64 = 64 - (Transition::STATE_ID_BITS + 1);
    const INFO_MASK: u64 = 0x000003FF_FFFFFFFF;
    fn new(match_wins: bool, sid: StateID, epsilons: Epsilons) -> Transition {
        let match_wins = if match_wins { 1 << Transition::MATCH_WINS_SHIFT } else { 0 };
        let sid = sid.as_u64() << Transition::STATE_ID_SHIFT;
        Transition(sid | match_wins | epsilons.0)
    }
    fn is_dead(self) -> bool {}
    fn match_wins(&self) -> bool {
        (self.0 >> Transition::MATCH_WINS_SHIFT & 1) == 1
    }
    fn state_id(&self) -> StateID {}
    fn set_state_id(&mut self, sid: StateID) {
        *self = Transition::new(self.match_wins(), sid, self.epsilons());
    }
    fn epsilons(&self) -> Epsilons {
        Epsilons(self.0 & Transition::INFO_MASK)
    }
}
