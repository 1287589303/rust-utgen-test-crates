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
#[derive(Clone)]
pub(crate) struct TransitionTable<T> {
    /// A contiguous region of memory representing the transition table in
    /// row-major order. The representation is dense. That is, every state
    /// has precisely the same number of transitions. The maximum number of
    /// transitions per state is 257 (256 for each possible byte value, plus 1
    /// for the special EOI transition). If a DFA has been instructed to use
    /// byte classes (the default), then the number of transitions is usually
    /// substantially fewer.
    ///
    /// In practice, T is either `Vec<u32>` or `&[u32]`.
    table: T,
    /// A set of equivalence classes, where a single equivalence class
    /// represents a set of bytes that never discriminate between a match
    /// and a non-match in the DFA. Each equivalence class corresponds to a
    /// single character in this DFA's alphabet, where the maximum number of
    /// characters is 257 (each possible value of a byte plus the special
    /// EOI transition). Consequently, the number of equivalence classes
    /// corresponds to the number of transitions for each DFA state. Note
    /// though that the *space* used by each DFA state in the transition table
    /// may be larger. The total space used by each DFA state is known as the
    /// stride.
    ///
    /// The only time the number of equivalence classes is fewer than 257 is if
    /// the DFA's kind uses byte classes (which is the default). Equivalence
    /// classes should generally only be disabled when debugging, so that
    /// the transitions themselves aren't obscured. Disabling them has no
    /// other benefit, since the equivalence class map is always used while
    /// searching. In the vast majority of cases, the number of equivalence
    /// classes is substantially smaller than 257, particularly when large
    /// Unicode classes aren't used.
    classes: ByteClasses,
    /// The stride of each DFA state, expressed as a power-of-two exponent.
    ///
    /// The stride of a DFA corresponds to the total amount of space used by
    /// each DFA state in the transition table. This may be bigger than the
    /// size of a DFA's alphabet, since the stride is always the smallest
    /// power of two greater than or equal to the alphabet size.
    ///
    /// While this wastes space, this avoids the need for integer division
    /// to convert between premultiplied state IDs and their corresponding
    /// indices. Instead, we can use simple bit-shifts.
    ///
    /// See the docs for the `stride2` method for more details.
    ///
    /// The minimum `stride2` value is `1` (corresponding to a stride of `2`)
    /// while the maximum `stride2` value is `9` (corresponding to a stride of
    /// `512`). The maximum is not `8` since the maximum alphabet size is `257`
    /// when accounting for the special EOI transition. However, an alphabet
    /// length of that size is exceptionally rare since the alphabet is shrunk
    /// into equivalence classes.
    stride2: usize,
}
#[derive(Clone, Copy, Debug)]
pub(crate) struct Special {
    /// The identifier of the last special state in a DFA. A state is special
    /// if and only if its identifier is less than or equal to `max`.
    pub(crate) max: StateID,
    /// The identifier of the quit state in a DFA. (There is no analogous field
    /// for the dead state since the dead state's ID is always zero, regardless
    /// of state ID size.)
    pub(crate) quit_id: StateID,
    /// The identifier of the first match state.
    pub(crate) min_match: StateID,
    /// The identifier of the last match state.
    pub(crate) max_match: StateID,
    /// The identifier of the first accelerated state.
    pub(crate) min_accel: StateID,
    /// The identifier of the last accelerated state.
    pub(crate) max_accel: StateID,
    /// The identifier of the first start state.
    pub(crate) min_start: StateID,
    /// The identifier of the last start state.
    pub(crate) max_start: StateID,
}
#[derive(Debug)]
pub struct DeserializeError(DeserializeErrorKind);
pub(crate) struct State<'a> {
    id: StateID,
    stride2: usize,
    transitions: &'a [StateID],
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
pub struct StateID(SmallIndex);
pub(crate) struct StateIter<'a, T> {
    tt: &'a TransitionTable<T>,
    it: iter::Enumerate<slice::Chunks<'a, StateID>>,
}
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Unit(UnitKind);
#[derive(Debug)]
pub(crate) struct StateTransitionIter<'a> {
    len: usize,
    it: iter::Enumerate<slice::Iter<'a, StateID>>,
}
#[derive(Clone, Copy)]
pub struct ByteClasses([u8; 256]);
impl<T: AsRef<[u32]>> TransitionTable<T> {
    fn write_to<E: Endian>(&self, mut dst: &mut [u8]) -> Result<usize, SerializeError> {}
    fn write_to_len(&self) -> usize {}
    fn validate(&self, dfa: &DFA<T>) -> Result<(), DeserializeError> {
        let sp = &dfa.special;
        for state in self.states() {
            if sp.is_special_state(state.id()) {
                let is_actually_special = sp.is_dead_state(state.id())
                    || sp.is_quit_state(state.id()) || sp.is_match_state(state.id())
                    || sp.is_start_state(state.id()) || sp.is_accel_state(state.id());
                if !is_actually_special {
                    return Err(
                        DeserializeError::generic(
                            "found dense state tagged as special but \
                         wasn't actually special",
                        ),
                    );
                }
                if sp.is_match_state(state.id()) && dfa.match_len(state.id()) == 0 {
                    return Err(
                        DeserializeError::generic(
                            "found match state with zero pattern IDs",
                        ),
                    );
                }
            }
            for (_, to) in state.transitions() {
                if !self.is_valid(to) {
                    return Err(
                        DeserializeError::generic(
                            "found invalid state ID in transition table",
                        ),
                    );
                }
            }
        }
        Ok(())
    }
    fn as_ref(&self) -> TransitionTable<&'_ [u32]> {}
    #[cfg(feature = "alloc")]
    fn to_owned(&self) -> TransitionTable<alloc::vec::Vec<u32>> {}
    fn state(&self, id: StateID) -> State<'_> {}
    fn states(&self) -> StateIter<'_, T> {
        StateIter {
            tt: self,
            it: self.table().chunks(self.stride()).enumerate(),
        }
    }
    fn to_index(&self, id: StateID) -> usize {}
    fn to_state_id(&self, index: usize) -> StateID {}
    #[cfg(feature = "dfa-build")]
    fn next_state_id(&self, id: StateID) -> StateID {}
    #[cfg(feature = "dfa-build")]
    fn prev_state_id(&self, id: StateID) -> StateID {}
    fn table(&self) -> &[StateID] {}
    fn len(&self) -> usize {}
    fn stride(&self) -> usize {}
    fn alphabet_len(&self) -> usize {}
    fn is_valid(&self, id: StateID) -> bool {
        let id = id.as_usize();
        id < self.table().len() && id % self.stride() == 0
    }
    fn memory_usage(&self) -> usize {}
}
impl Special {
    #[cfg(feature = "dfa-build")]
    pub(crate) fn new() -> Special {}
    #[cfg(feature = "dfa-build")]
    pub(crate) fn remap(&self, map: impl Fn(StateID) -> StateID) -> Special {}
    pub(crate) fn from_bytes(
        mut slice: &[u8],
    ) -> Result<(Special, usize), DeserializeError> {}
    pub(crate) fn validate(&self) -> Result<(), DeserializeError> {}
    pub(crate) fn validate_state_len(
        &self,
        len: usize,
        stride2: usize,
    ) -> Result<(), DeserializeError> {}
    pub(crate) fn write_to<E: Endian>(
        &self,
        dst: &mut [u8],
    ) -> Result<usize, SerializeError> {}
    pub(crate) fn write_to_len(&self) -> usize {}
    #[cfg(feature = "dfa-build")]
    pub(crate) fn set_max(&mut self) {}
    #[cfg(feature = "dfa-build")]
    pub(crate) fn set_no_special_start_states(&mut self) {}
    #[inline]
    pub(crate) fn is_special_state(&self, id: StateID) -> bool {
        id <= self.max
    }
    #[inline]
    pub(crate) fn is_dead_state(&self, id: StateID) -> bool {
        id == DEAD
    }
    #[inline]
    pub(crate) fn is_quit_state(&self, id: StateID) -> bool {
        !self.is_dead_state(id) && self.quit_id == id
    }
    #[inline]
    pub(crate) fn is_match_state(&self, id: StateID) -> bool {
        !self.is_dead_state(id) && self.min_match <= id && id <= self.max_match
    }
    #[inline]
    pub(crate) fn is_accel_state(&self, id: StateID) -> bool {
        !self.is_dead_state(id) && self.min_accel <= id && id <= self.max_accel
    }
    #[inline]
    pub(crate) fn is_start_state(&self, id: StateID) -> bool {
        !self.is_dead_state(id) && self.min_start <= id && id <= self.max_start
    }
    #[inline]
    pub(crate) fn match_len(&self, stride: usize) -> usize {}
    #[inline]
    pub(crate) fn matches(&self) -> bool {}
    #[cfg(feature = "dfa-build")]
    pub(crate) fn accel_len(&self, stride: usize) -> usize {}
    #[inline]
    pub(crate) fn accels(&self) -> bool {}
    #[inline]
    pub(crate) fn starts(&self) -> bool {}
}
impl DeserializeError {
    pub(crate) fn generic(msg: &'static str) -> DeserializeError {
        DeserializeError(DeserializeErrorKind::Generic {
            msg,
        })
    }
    pub(crate) fn buffer_too_small(what: &'static str) -> DeserializeError {}
    fn invalid_usize(what: &'static str) -> DeserializeError {}
    fn version_mismatch(expected: u32, found: u32) -> DeserializeError {}
    fn endian_mismatch(expected: u32, found: u32) -> DeserializeError {}
    fn alignment_mismatch(alignment: usize, address: usize) -> DeserializeError {}
    fn label_mismatch(expected: &'static str) -> DeserializeError {}
    fn arithmetic_overflow(what: &'static str) -> DeserializeError {}
    fn pattern_id_error(err: PatternIDError, what: &'static str) -> DeserializeError {}
    pub(crate) fn state_id_error(
        err: StateIDError,
        what: &'static str,
    ) -> DeserializeError {}
}
impl<'a> State<'a> {
    pub(crate) fn transitions(&self) -> StateTransitionIter<'_> {
        StateTransitionIter {
            len: self.transitions.len(),
            it: self.transitions.iter().enumerate(),
        }
    }
    pub(crate) fn sparse_transitions(&self) -> StateSparseTransitionIter<'_> {}
    pub(crate) fn id(&self) -> StateID {
        self.id
    }
    #[cfg(feature = "dfa-build")]
    fn accelerate(&self, classes: &ByteClasses) -> Option<Accel> {}
}
