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
pub unsafe trait Automaton {
    fn next_state(&self, current: StateID, input: u8) -> StateID;
    unsafe fn next_state_unchecked(&self, current: StateID, input: u8) -> StateID;
    fn next_eoi_state(&self, current: StateID) -> StateID;
    fn start_state(&self, config: &start::Config) -> Result<StateID, StartError>;
    fn start_state_forward(&self, input: &Input<'_>) -> Result<StateID, MatchError>;
    fn start_state_reverse(&self, input: &Input<'_>) -> Result<StateID, MatchError>;
    #[inline]
    fn universal_start_state(&self, _mode: Anchored) -> Option<StateID>;
    fn is_special_state(&self, id: StateID) -> bool;
    fn is_dead_state(&self, id: StateID) -> bool;
    fn is_quit_state(&self, id: StateID) -> bool;
    fn is_match_state(&self, id: StateID) -> bool;
    fn is_start_state(&self, id: StateID) -> bool;
    fn is_accel_state(&self, id: StateID) -> bool;
    fn pattern_len(&self) -> usize;
    fn match_len(&self, id: StateID) -> usize;
    fn match_pattern(&self, id: StateID, index: usize) -> PatternID;
    fn has_empty(&self) -> bool;
    fn is_utf8(&self) -> bool;
    fn is_always_start_anchored(&self) -> bool;
    #[inline]
    fn accelerator(&self, _id: StateID) -> &[u8];
    #[inline]
    fn get_prefilter(&self) -> Option<&Prefilter>;
    #[inline]
    fn try_search_fwd(&self, input: &Input<'_>) -> Result<Option<HalfMatch>, MatchError>;
    #[inline]
    fn try_search_rev(&self, input: &Input<'_>) -> Result<Option<HalfMatch>, MatchError>;
    #[inline]
    fn try_search_overlapping_fwd(
        &self,
        input: &Input<'_>,
        state: &mut OverlappingState,
    ) -> Result<(), MatchError>;
    #[inline]
    fn try_search_overlapping_rev(
        &self,
        input: &Input<'_>,
        state: &mut OverlappingState,
    ) -> Result<(), MatchError>;
    #[cfg(feature = "alloc")]
    #[inline]
    fn try_which_overlapping_matches(
        &self,
        input: &Input<'_>,
        patset: &mut PatternSet,
    ) -> Result<(), MatchError>;
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
#[derive(Debug)]
pub struct DeserializeError(DeserializeErrorKind);
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
pub(crate) struct Accels<A> {
    /// A length prefixed slice of contiguous accelerators. See the top comment
    /// in this module for more details on how we can jump from a DFA's state
    /// ID to an accelerator in this list.
    ///
    /// The first 4 bytes always correspond to the number of accelerators
    /// that follow.
    accels: A,
}
pub(crate) struct State<'a> {
    id: StateID,
    stride2: usize,
    transitions: &'a [StateID],
}
#[derive(Clone)]
pub(crate) struct StartTable<T> {
    /// The initial start state IDs.
    ///
    /// In practice, T is either `Vec<u32>` or `&[u32]`.
    ///
    /// The first `2 * stride` (currently always 8) entries always correspond
    /// to the starts states for the entire DFA, with the first 4 entries being
    /// for unanchored searches and the second 4 entries being for anchored
    /// searches. To keep things simple, we always use 8 entries even if the
    /// `StartKind` is not both.
    ///
    /// After that, there are `stride * patterns` state IDs, where `patterns`
    /// may be zero in the case of a DFA with no patterns or in the case where
    /// the DFA was built without enabling starting states for each pattern.
    table: T,
    /// The starting state configuration supported. When 'both', both
    /// unanchored and anchored searches work. When 'unanchored', anchored
    /// searches panic. When 'anchored', unanchored searches panic.
    kind: StartKind,
    /// The start state configuration for every possible byte.
    start_map: StartByteMap,
    /// The number of starting state IDs per pattern.
    stride: usize,
    /// The total number of patterns for which starting states are encoded.
    /// This is `None` for DFAs that were built without start states for each
    /// pattern. Thus, one cannot use this field to say how many patterns
    /// are in the DFA in all cases. It is specific to how many patterns are
    /// represented in this start table.
    pattern_len: Option<usize>,
    /// The universal starting state for unanchored searches. This is only
    /// present when the DFA supports unanchored searches and when all starting
    /// state IDs for an unanchored search are equivalent.
    universal_start_unanchored: Option<StateID>,
    /// The universal starting state for anchored searches. This is only
    /// present when the DFA supports anchored searches and when all starting
    /// state IDs for an anchored search are equivalent.
    universal_start_anchored: Option<StateID>,
}
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
pub(crate) struct Flags {
    /// Whether the DFA can match the empty string. When this is false, all
    /// matches returned by this DFA are guaranteed to have non-zero length.
    pub(crate) has_empty: bool,
    /// Whether the DFA should only produce matches with spans that correspond
    /// to valid UTF-8. This also includes omitting any zero-width matches that
    /// split the UTF-8 encoding of a codepoint.
    pub(crate) is_utf8: bool,
    /// Whether the DFA is always anchored or not, regardless of `Input`
    /// configuration. This is useful for avoiding a reverse scan even when
    /// executing unanchored searches.
    pub(crate) is_always_start_anchored: bool,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
pub(crate) struct StateIter<'a, T> {
    tt: &'a TransitionTable<T>,
    it: iter::Enumerate<slice::Chunks<'a, StateID>>,
}
#[derive(Clone, Debug)]
pub(crate) struct ByteSet([bool; 256]);
#[derive(Clone)]
struct StartTable<T> {
    /// The initial start state IDs as a contiguous table of native endian
    /// encoded integers, represented by `S`.
    ///
    /// In practice, T is either Vec<u8> or &[u8] and has no alignment
    /// requirements.
    ///
    /// The first `2 * stride` (currently always 8) entries always correspond
    /// to the starts states for the entire DFA, with the first 4 entries being
    /// for unanchored searches and the second 4 entries being for anchored
    /// searches. To keep things simple, we always use 8 entries even if the
    /// `StartKind` is not both.
    ///
    /// After that, there are `stride * patterns` state IDs, where `patterns`
    /// may be zero in the case of a DFA with no patterns or in the case where
    /// the DFA was built without enabling starting states for each pattern.
    table: T,
    /// The starting state configuration supported. When 'both', both
    /// unanchored and anchored searches work. When 'unanchored', anchored
    /// searches panic. When 'anchored', unanchored searches panic.
    kind: StartKind,
    /// The start state configuration for every possible byte.
    start_map: StartByteMap,
    /// The number of starting state IDs per pattern.
    stride: usize,
    /// The total number of patterns for which starting states are encoded.
    /// This is `None` for DFAs that were built without start states for each
    /// pattern. Thus, one cannot use this field to say how many patterns
    /// are in the DFA in all cases. It is specific to how many patterns are
    /// represented in this start table.
    pattern_len: Option<usize>,
    /// The universal starting state for unanchored searches. This is only
    /// present when the DFA supports unanchored searches and when all starting
    /// state IDs for an unanchored search are equivalent.
    universal_start_unanchored: Option<StateID>,
    /// The universal starting state for anchored searches. This is only
    /// present when the DFA supports anchored searches and when all starting
    /// state IDs for an anchored search are equivalent.
    universal_start_anchored: Option<StateID>,
}
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct ByteSet {
    bits: BitSet,
}
#[derive(Clone, Debug)]
pub struct Prefilter {
    #[cfg(not(feature = "alloc"))]
    _unused: (),
    #[cfg(feature = "alloc")]
    pre: Arc<dyn PrefilterI>,
    #[cfg(feature = "alloc")]
    is_fast: bool,
    #[cfg(feature = "alloc")]
    max_needle_len: usize,
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
impl<'a> DFA<&'a [u32]> {
    pub fn from_bytes(
        slice: &'a [u8],
    ) -> Result<(DFA<&'a [u32]>, usize), DeserializeError> {
        let (dfa, nread) = unsafe { DFA::from_bytes_unchecked(slice)? };
        dfa.tt.validate(&dfa)?;
        dfa.st.validate(&dfa)?;
        dfa.ms.validate(&dfa)?;
        dfa.accels.validate()?;
        for state in dfa.states() {
            if dfa.is_accel_state(state.id()) {
                let index = dfa.accelerator_index(state.id());
                if index >= dfa.accels.len() {
                    return Err(
                        DeserializeError::generic(
                            "found DFA state with invalid accelerator index",
                        ),
                    );
                }
                let needles = dfa.accels.needles(index);
                if !(1 <= needles.len() && needles.len() <= 3) {
                    return Err(
                        DeserializeError::generic(
                            "accelerator needles has invalid length",
                        ),
                    );
                }
            }
        }
        Ok((dfa, nread))
    }
    pub unsafe fn from_bytes_unchecked(
        slice: &'a [u8],
    ) -> Result<(DFA<&'a [u32]>, usize), DeserializeError> {
        let mut nr = 0;
        nr += wire::skip_initial_padding(slice);
        wire::check_alignment::<StateID>(&slice[nr..])?;
        nr += wire::read_label(&slice[nr..], LABEL)?;
        nr += wire::read_endianness_check(&slice[nr..])?;
        nr += wire::read_version(&slice[nr..], VERSION)?;
        let _unused = wire::try_read_u32(&slice[nr..], "unused space")?;
        nr += size_of::<u32>();
        let (flags, nread) = Flags::from_bytes(&slice[nr..])?;
        nr += nread;
        let (tt, nread) = TransitionTable::from_bytes_unchecked(&slice[nr..])?;
        nr += nread;
        let (st, nread) = StartTable::from_bytes_unchecked(&slice[nr..])?;
        nr += nread;
        let (ms, nread) = MatchStates::from_bytes_unchecked(&slice[nr..])?;
        nr += nread;
        let (special, nread) = Special::from_bytes(&slice[nr..])?;
        nr += nread;
        special.validate_state_len(tt.len(), tt.stride2)?;
        let (accels, nread) = Accels::from_bytes_unchecked(&slice[nr..])?;
        nr += nread;
        let (quitset, nread) = ByteSet::from_bytes(&slice[nr..])?;
        nr += nread;
        let pre = None;
        Ok((
            DFA {
                tt,
                st,
                ms,
                special,
                accels,
                pre,
                quitset,
                flags,
            },
            nr,
        ))
    }
    fn write_to<E: Endian>(&self, mut dst: &mut [u8]) -> Result<usize, SerializeError> {}
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
impl<T: AsRef<[u32]>> MatchStates<T> {
    fn write_to<E: Endian>(&self, mut dst: &mut [u8]) -> Result<usize, SerializeError> {}
    fn write_to_len(&self) -> usize {}
    fn validate(&self, dfa: &DFA<T>) -> Result<(), DeserializeError> {
        if self.len() != dfa.special.match_len(dfa.stride()) {
            return Err(DeserializeError::generic("match state length mismatch"));
        }
        for si in 0..self.len() {
            let start = self.slices()[si * 2].as_usize();
            let len = self.slices()[si * 2 + 1].as_usize();
            if start >= self.pattern_ids().len() {
                return Err(DeserializeError::generic("invalid pattern ID start offset"));
            }
            if start + len > self.pattern_ids().len() {
                return Err(DeserializeError::generic("invalid pattern ID length"));
            }
            for mi in 0..len {
                let pid = self.pattern_id(si, mi);
                if pid.as_usize() >= self.pattern_len {
                    return Err(DeserializeError::generic("invalid pattern ID"));
                }
            }
        }
        Ok(())
    }
    #[cfg(feature = "dfa-build")]
    fn to_map(&self, dfa: &DFA<T>) -> BTreeMap<StateID, Vec<PatternID>> {}
    fn as_ref(&self) -> MatchStates<&'_ [u32]> {}
    #[cfg(feature = "alloc")]
    fn to_owned(&self) -> MatchStates<alloc::vec::Vec<u32>> {}
    fn match_state_id(&self, dfa: &DFA<T>, index: usize) -> StateID {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn pattern_id(&self, state_index: usize, match_index: usize) -> PatternID {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn pattern_len(&self, state_index: usize) -> usize {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn pattern_id_slice(&self, state_index: usize) -> &[PatternID] {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn slices(&self) -> &[PatternID] {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn len(&self) -> usize {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn pattern_ids(&self) -> &[PatternID] {}
    fn memory_usage(&self) -> usize {}
}
impl<A: AsRef<[AccelTy]>> Accels<A> {
    #[cfg(feature = "alloc")]
    pub fn to_owned(&self) -> Accels<alloc::vec::Vec<AccelTy>> {}
    pub fn as_ref(&self) -> Accels<&[AccelTy]> {}
    pub fn as_bytes(&self) -> &[u8] {}
    pub fn memory_usage(&self) -> usize {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub fn needles(&self, i: usize) -> &[u8] {
        if i >= self.len() {
            panic!("invalid accelerator index {}", i);
        }
        let bytes = self.as_bytes();
        let offset = ACCEL_TY_SIZE + i * ACCEL_CAP;
        let len = usize::from(bytes[offset]);
        &bytes[offset + 1..offset + 1 + len]
    }
    pub fn len(&self) -> usize {
        usize::try_from(self.accels.as_ref()[0]).unwrap()
    }
    fn get(&self, i: usize) -> Option<Accel> {}
    fn iter(&self) -> IterAccels<'_, A> {}
    pub fn write_to<E: Endian>(&self, dst: &mut [u8]) -> Result<usize, SerializeError> {}
    pub fn validate(&self) -> Result<(), DeserializeError> {
        for chunk in self.as_bytes()[ACCEL_TY_SIZE..].chunks(ACCEL_CAP) {
            let _ = Accel::from_slice(chunk)?;
        }
        Ok(())
    }
    pub fn write_to_len(&self) -> usize {}
}
impl<T: AsRef<[u32]>> DFA<T> {
    pub(crate) fn special(&self) -> &Special {}
    #[cfg(feature = "dfa-build")]
    pub(crate) fn special_mut(&mut self) -> &mut Special {}
    pub(crate) fn quitset(&self) -> &ByteSet {}
    pub(crate) fn flags(&self) -> &Flags {}
    pub(crate) fn states(&self) -> StateIter<'_, T> {
        self.tt.states()
    }
    pub(crate) fn state_len(&self) -> usize {}
    #[cfg(feature = "dfa-build")]
    pub(crate) fn pattern_id_slice(&self, id: StateID) -> &[PatternID] {}
    pub(crate) fn match_pattern_len(&self, id: StateID) -> usize {}
    pub(crate) fn pattern_len(&self) -> usize {}
    #[cfg(feature = "dfa-build")]
    pub(crate) fn pattern_map(&self) -> BTreeMap<StateID, Vec<PatternID>> {}
    #[cfg(feature = "dfa-build")]
    pub(crate) fn quit_id(&self) -> StateID {}
    pub(crate) fn to_index(&self, id: StateID) -> usize {}
    #[cfg(feature = "dfa-build")]
    pub(crate) fn to_state_id(&self, index: usize) -> StateID {}
    pub(crate) fn starts(&self) -> StartStateIter<'_> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn match_state_index(&self, id: StateID) -> usize {}
    fn accelerator_index(&self, id: StateID) -> usize {
        let min = self.special().min_accel.as_usize();
        self.to_index(StateID::new_unchecked(id.as_usize() - min))
    }
    fn accels(&self) -> Accels<&[u32]> {}
    fn trans(&self) -> &[StateID] {}
}
impl<'a> State<'a> {
    pub(crate) fn transitions(&self) -> StateTransitionIter<'_> {}
    pub(crate) fn sparse_transitions(&self) -> StateSparseTransitionIter<'_> {}
    pub(crate) fn id(&self) -> StateID {
        self.id
    }
    #[cfg(feature = "dfa-build")]
    fn accelerate(&self, classes: &ByteClasses) -> Option<Accel> {}
}
impl<T: AsRef<[u32]>> StartTable<T> {
    fn write_to<E: Endian>(&self, mut dst: &mut [u8]) -> Result<usize, SerializeError> {}
    fn write_to_len(&self) -> usize {}
    fn validate(&self, dfa: &DFA<T>) -> Result<(), DeserializeError> {
        let tt = &dfa.tt;
        if !self.universal_start_unanchored.map_or(true, |s| tt.is_valid(s)) {
            return Err(
                DeserializeError::generic(
                    "found invalid universal unanchored starting state ID",
                ),
            );
        }
        if !self.universal_start_anchored.map_or(true, |s| tt.is_valid(s)) {
            return Err(
                DeserializeError::generic(
                    "found invalid universal anchored starting state ID",
                ),
            );
        }
        for &id in self.table() {
            if !tt.is_valid(id) {
                return Err(DeserializeError::generic("found invalid starting state ID"));
            }
        }
        Ok(())
    }
    fn as_ref(&self) -> StartTable<&'_ [u32]> {}
    #[cfg(feature = "alloc")]
    fn to_owned(&self) -> StartTable<alloc::vec::Vec<u32>> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn start(&self, anchored: Anchored, start: Start) -> Result<StateID, StartError> {}
    fn iter(&self) -> StartStateIter<'_> {}
    fn table(&self) -> &[StateID] {}
    fn memory_usage(&self) -> usize {}
}
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
    fn states(&self) -> StateIter<'_, T> {}
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
    fn is_valid(&self, id: StateID) -> bool {}
    fn memory_usage(&self) -> usize {}
}
