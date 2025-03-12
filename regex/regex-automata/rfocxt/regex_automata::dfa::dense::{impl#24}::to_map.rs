#[cfg(feature = "alloc")]
pub(crate) type OwnedDFA = DFA<alloc::vec::Vec<u32>>;
#[cfg(feature = "dfa-build")]
use core::cmp;
use core::{fmt, iter, mem::size_of, slice};
#[cfg(feature = "dfa-build")]
use alloc::{
    collections::{BTreeMap, BTreeSet},
    vec, vec::Vec,
};
#[cfg(feature = "dfa-build")]
use crate::{
    dfa::{accel::Accel, determinize, minimize::Minimizer, remapper::Remapper, sparse},
    nfa::thompson, util::{look::LookMatcher, search::MatchKind},
};
use crate::{
    dfa::{
        accel::Accels, automaton::{fmt_state_indicator, Automaton, StartError},
        special::Special, start::StartKind, DEAD,
    },
    util::{
        alphabet::{self, ByteClasses, ByteSet},
        int::{Pointer, Usize},
        prefilter::Prefilter, primitives::{PatternID, StateID},
        search::Anchored, start::{self, Start, StartByteMap},
        wire::{self, DeserializeError, Endian, SerializeError},
    },
};
const LABEL: &str = "rust-regex-automata-dfa-dense";
const VERSION: u32 = 2;
#[derive(Clone, Debug)]
struct MatchStates<T> {
    /// Slices is a flattened sequence of pairs, where each pair points to a
    /// sub-slice of pattern_ids. The first element of the pair is an offset
    /// into pattern_ids and the second element of the pair is the number
    /// of 32-bit pattern IDs starting at that position. That is, each pair
    /// corresponds to a single DFA match state and its corresponding match
    /// IDs. The number of pairs always corresponds to the number of distinct
    /// DFA match states.
    ///
    /// In practice, T is either Vec<u32> or &[u32].
    slices: T,
    /// A flattened sequence of pattern IDs for each DFA match state. The only
    /// way to correctly read this sequence is indirectly via `slices`.
    ///
    /// In practice, T is either Vec<u32> or &[u32].
    pattern_ids: T,
    /// The total number of unique patterns represented by these match states.
    pattern_len: usize,
}
#[derive(Clone)]
pub struct DFA<T> {
    /// The transition table for this DFA. This includes the transitions
    /// themselves, along with the stride, number of states and the equivalence
    /// class mapping.
    tt: TransitionTable<T>,
    /// The set of starting state identifiers for this DFA. The starting state
    /// IDs act as pointers into the transition table. The specific starting
    /// state chosen for each search is dependent on the context at which the
    /// search begins.
    st: StartTable<T>,
    /// The set of match states and the patterns that match for each
    /// corresponding match state.
    ///
    /// This structure is technically only needed because of support for
    /// multi-regexes. Namely, multi-regexes require answering not just whether
    /// a match exists, but _which_ patterns match. So we need to store the
    /// matching pattern IDs for each match state. We do this even when there
    /// is only one pattern for the sake of simplicity. In practice, this uses
    /// up very little space for the case of one pattern.
    ms: MatchStates<T>,
    /// Information about which states are "special." Special states are states
    /// that are dead, quit, matching, starting or accelerated. For more info,
    /// see the docs for `Special`.
    special: Special,
    /// The accelerators for this DFA.
    ///
    /// If a state is accelerated, then there exist only a small number of
    /// bytes that can cause the DFA to leave the state. This permits searching
    /// to use optimized routines to find those specific bytes instead of using
    /// the transition table.
    ///
    /// All accelerated states exist in a contiguous range in the DFA's
    /// transition table. See dfa/special.rs for more details on how states are
    /// arranged.
    accels: Accels<T>,
    /// Any prefilter attached to this DFA.
    ///
    /// Note that currently prefilters are not serialized. When deserializing
    /// a DFA from bytes, this is always set to `None`.
    pre: Option<Prefilter>,
    /// The set of "quit" bytes for this DFA.
    ///
    /// This is only used when computing the start state for a particular
    /// position in a haystack. Namely, in the case where there is a quit
    /// byte immediately before the start of the search, this set needs to be
    /// explicitly consulted. In all other cases, quit bytes are detected by
    /// the DFA itself, by transitioning all quit bytes to a special "quit
    /// state."
    quitset: ByteSet,
    /// Various flags describing the behavior of this DFA.
    flags: Flags,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
impl<T: AsRef<[u32]>> MatchStates<T> {
    fn write_to<E: Endian>(&self, mut dst: &mut [u8]) -> Result<usize, SerializeError> {}
    fn write_to_len(&self) -> usize {}
    fn validate(&self, dfa: &DFA<T>) -> Result<(), DeserializeError> {}
    #[cfg(feature = "dfa-build")]
    fn to_map(&self, dfa: &DFA<T>) -> BTreeMap<StateID, Vec<PatternID>> {
        let mut map = BTreeMap::new();
        for i in 0..self.len() {
            let mut pids = vec![];
            for j in 0..self.pattern_len(i) {
                pids.push(self.pattern_id(i, j));
            }
            map.insert(self.match_state_id(dfa, i), pids);
        }
        map
    }
    fn as_ref(&self) -> MatchStates<&'_ [u32]> {}
    #[cfg(feature = "alloc")]
    fn to_owned(&self) -> MatchStates<alloc::vec::Vec<u32>> {}
    fn match_state_id(&self, dfa: &DFA<T>, index: usize) -> StateID {
        assert!(dfa.special.matches(), "no match states to index");
        let stride2 = u32::try_from(dfa.stride2()).unwrap();
        let offset = index.checked_shl(stride2).unwrap();
        let id = dfa.special.min_match.as_usize().checked_add(offset).unwrap();
        let sid = StateID::new(id).unwrap();
        assert!(dfa.is_match_state(sid));
        sid
    }
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn pattern_id(&self, state_index: usize, match_index: usize) -> PatternID {
        self.pattern_id_slice(state_index)[match_index]
    }
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn pattern_len(&self, state_index: usize) -> usize {
        self.slices()[state_index * 2 + 1].as_usize()
    }
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn pattern_id_slice(&self, state_index: usize) -> &[PatternID] {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn slices(&self) -> &[PatternID] {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn len(&self) -> usize {
        assert_eq!(0, self.slices().len() % 2);
        self.slices().len() / 2
    }
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn pattern_ids(&self) -> &[PatternID] {}
    fn memory_usage(&self) -> usize {}
}
