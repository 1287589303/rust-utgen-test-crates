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
#[derive(Debug)]
struct SparseTransitionIter<'a> {
    it: core::iter::Enumerate<core::slice::Iter<'a, Transition>>,
    cur: Option<(u8, u8, Transition)>,
}
#[derive(Clone, Copy, Eq, PartialEq)]
struct Transition(u64);
#[derive(Clone)]
struct Transition {
    /// The byte range.
    range: Utf8Range,
    /// The next state to transition to.
    next_id: StateID,
}
#[derive(Clone, Copy)]
struct Transition {
    byte: u8,
    next: StateID,
}
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Transition {
    /// The inclusive start of the byte range.
    pub start: u8,
    /// The inclusive end of the byte range.
    pub end: u8,
    /// The identifier of the state to transition to.
    pub next: StateID,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
impl<'a> Iterator for SparseTransitionIter<'a> {
    type Item = (u8, u8, Transition);
    fn next(&mut self) -> Option<(u8, u8, Transition)> {
        while let Some((b, &trans)) = self.it.next() {
            let b = b.as_u8();
            let (prev_start, prev_end, prev_trans) = match self.cur {
                Some(t) => t,
                None => {
                    self.cur = Some((b, b, trans));
                    continue;
                }
            };
            if prev_trans == trans {
                self.cur = Some((prev_start, b, prev_trans));
            } else {
                self.cur = Some((b, b, trans));
                if prev_trans.state_id() != DEAD {
                    return Some((prev_start, prev_end, prev_trans));
                }
            }
        }
        if let Some((start, end, trans)) = self.cur.take() {
            if trans.state_id() != DEAD {
                return Some((start, end, trans));
            }
        }
        None
    }
}
impl Transition {
    const STATE_ID_BITS: u64 = 21;
    const STATE_ID_SHIFT: u64 = 64 - Transition::STATE_ID_BITS;
    const STATE_ID_LIMIT: u64 = 1 << Transition::STATE_ID_BITS;
    const MATCH_WINS_SHIFT: u64 = 64 - (Transition::STATE_ID_BITS + 1);
    const INFO_MASK: u64 = 0x000003FF_FFFFFFFF;
    fn new(match_wins: bool, sid: StateID, epsilons: Epsilons) -> Transition {}
    fn is_dead(self) -> bool {}
    fn match_wins(&self) -> bool {}
    fn state_id(&self) -> StateID {
        StateID::new_unchecked((self.0 >> Transition::STATE_ID_SHIFT).as_usize())
    }
    fn set_state_id(&mut self, sid: StateID) {}
    fn epsilons(&self) -> Epsilons {}
}
