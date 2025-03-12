use alloc::{vec, vec::Vec};
use crate::{
    nfa::thompson::{self, BuildError, State, NFA},
    util::{
        captures::Captures, empty, iter, prefilter::Prefilter,
        primitives::{NonMaxUsize, PatternID, SmallIndex, StateID},
        search::{Anchored, HalfMatch, Input, Match, MatchError, Span},
    },
};
fn div_ceil(lhs: usize, rhs: usize) -> usize {
    if lhs % rhs == 0 { lhs / rhs } else { (lhs / rhs) + 1 }
}
