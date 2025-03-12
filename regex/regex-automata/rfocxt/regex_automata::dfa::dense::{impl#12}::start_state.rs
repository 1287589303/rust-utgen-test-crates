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
    fn start_state_forward(&self, input: &Input<'_>) -> Result<StateID, MatchError> {
        let config = start::Config::from_input_forward(input);
        self.start_state(&config)
            .map_err(|err| match err {
                StartError::Quit { byte } => {
                    let offset = input
                        .start()
                        .checked_sub(1)
                        .expect("no quit in start without look-behind");
                    MatchError::quit(byte, offset)
                }
                StartError::UnsupportedAnchored { mode } => {
                    MatchError::unsupported_anchored(mode)
                }
            })
    }
    fn start_state_reverse(&self, input: &Input<'_>) -> Result<StateID, MatchError> {
        let config = start::Config::from_input_reverse(input);
        self.start_state(&config)
            .map_err(|err| match err {
                StartError::Quit { byte } => {
                    let offset = input.end();
                    MatchError::quit(byte, offset)
                }
                StartError::UnsupportedAnchored { mode } => {
                    MatchError::unsupported_anchored(mode)
                }
            })
    }
    #[inline]
    fn universal_start_state(&self, _mode: Anchored) -> Option<StateID> {
        None
    }
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
    fn accelerator(&self, _id: StateID) -> &[u8] {
        &[]
    }
    #[inline]
    fn get_prefilter(&self) -> Option<&Prefilter> {
        None
    }
    #[inline]
    fn try_search_fwd(
        &self,
        input: &Input<'_>,
    ) -> Result<Option<HalfMatch>, MatchError> {
        let utf8empty = self.has_empty() && self.is_utf8();
        let hm = match search::find_fwd(&self, input)? {
            None => return Ok(None),
            Some(hm) if !utf8empty => return Ok(Some(hm)),
            Some(hm) => hm,
        };
        empty::skip_splits_fwd(
            input,
            hm,
            hm.offset(),
            |input| {
                let got = search::find_fwd(&self, input)?;
                Ok(got.map(|hm| (hm, hm.offset())))
            },
        )
    }
    #[inline]
    fn try_search_rev(
        &self,
        input: &Input<'_>,
    ) -> Result<Option<HalfMatch>, MatchError> {
        let utf8empty = self.has_empty() && self.is_utf8();
        let hm = match search::find_rev(self, input)? {
            None => return Ok(None),
            Some(hm) if !utf8empty => return Ok(Some(hm)),
            Some(hm) => hm,
        };
        empty::skip_splits_rev(
            input,
            hm,
            hm.offset(),
            |input| {
                let got = search::find_rev(self, input)?;
                Ok(got.map(|hm| (hm, hm.offset())))
            },
        )
    }
    #[inline]
    fn try_search_overlapping_fwd(
        &self,
        input: &Input<'_>,
        state: &mut OverlappingState,
    ) -> Result<(), MatchError> {
        let utf8empty = self.has_empty() && self.is_utf8();
        search::find_overlapping_fwd(self, input, state)?;
        match state.get_match() {
            None => Ok(()),
            Some(_) if !utf8empty => Ok(()),
            Some(_) => {
                skip_empty_utf8_splits_overlapping(
                    input,
                    state,
                    |input, state| { search::find_overlapping_fwd(self, input, state) },
                )
            }
        }
    }
    #[inline]
    fn try_search_overlapping_rev(
        &self,
        input: &Input<'_>,
        state: &mut OverlappingState,
    ) -> Result<(), MatchError> {
        let utf8empty = self.has_empty() && self.is_utf8();
        search::find_overlapping_rev(self, input, state)?;
        match state.get_match() {
            None => Ok(()),
            Some(_) if !utf8empty => Ok(()),
            Some(_) => {
                skip_empty_utf8_splits_overlapping(
                    input,
                    state,
                    |input, state| { search::find_overlapping_rev(self, input, state) },
                )
            }
        }
    }
    #[cfg(feature = "alloc")]
    #[inline]
    fn try_which_overlapping_matches(
        &self,
        input: &Input<'_>,
        patset: &mut PatternSet,
    ) -> Result<(), MatchError> {
        let mut state = OverlappingState::start();
        while let Some(m) = {
            self.try_search_overlapping_fwd(input, &mut state)?;
            state.get_match()
        } {
            let _ = patset.insert(m.pattern());
            if patset.is_full() || input.get_earliest() {
                break;
            }
        }
        Ok(())
    }
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
#[derive(Clone, Debug)]
pub struct Config {
    look_behind: Option<u8>,
    anchored: Anchored,
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
pub(crate) struct StartByteMap {
    map: [Start; 256],
}
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct ByteSet {
    bits: BitSet,
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
#[derive(Clone, Debug)]
pub(crate) struct ByteSet([bool; 256]);
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
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
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
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum StartError {
    /// An error that occurs when a starting configuration's look-behind byte
    /// is in this DFA's quit set.
    Quit {
        /// The quit byte that was found.
        byte: u8,
    },
    /// An error that occurs when the caller requests an anchored mode that
    /// isn't supported by the DFA.
    UnsupportedAnchored {
        /// The anchored mode given that is unsupported.
        mode: Anchored,
    },
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Start {
    /// This occurs when the starting position is not any of the ones below.
    NonWordByte = 0,
    /// This occurs when the byte immediately preceding the start of the search
    /// is an ASCII word byte.
    WordByte = 1,
    /// This occurs when the starting position of the search corresponds to the
    /// beginning of the haystack.
    Text = 2,
    /// This occurs when the byte immediately preceding the start of the search
    /// is a line terminator. Specifically, `\n`.
    LineLF = 3,
    /// This occurs when the byte immediately preceding the start of the search
    /// is a line terminator. Specifically, `\r`.
    LineCR = 4,
    /// This occurs when a custom line terminator has been set via a
    /// `LookMatcher`, and when that line terminator is neither a `\r` or a
    /// `\n`.
    ///
    /// If the custom line terminator is a word byte, then this start
    /// configuration is still selected. DFAs that implement word boundary
    /// assertions will likely need to check whether the custom line terminator
    /// is a word byte, in which case, it should behave as if the byte
    /// satisfies `\b` in addition to multi-line anchors.
    CustomLineTerminator = 5,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Anchored {
    /// Run an unanchored search. This means a match may occur anywhere at or
    /// after the start position of the search.
    ///
    /// This search can return a match for any pattern in the regex.
    No,
    /// Run an anchored search. This means that a match must begin at the
    /// start position of the search.
    ///
    /// This search can return a match for any pattern in the regex.
    Yes,
    /// Run an anchored search for a specific pattern. This means that a match
    /// must be for the given pattern and must begin at the start position of
    /// the search.
    Pattern(PatternID),
}
unsafe impl<T: AsRef<[u32]>> Automaton for DFA<T> {
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn is_special_state(&self, id: StateID) -> bool {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn is_dead_state(&self, id: StateID) -> bool {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn is_quit_state(&self, id: StateID) -> bool {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn is_match_state(&self, id: StateID) -> bool {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn is_start_state(&self, id: StateID) -> bool {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn is_accel_state(&self, id: StateID) -> bool {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn next_state(&self, current: StateID, input: u8) -> StateID {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    unsafe fn next_state_unchecked(&self, current: StateID, byte: u8) -> StateID {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn next_eoi_state(&self, current: StateID) -> StateID {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn pattern_len(&self) -> usize {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn match_len(&self, id: StateID) -> usize {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn match_pattern(&self, id: StateID, match_index: usize) -> PatternID {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn has_empty(&self) -> bool {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn is_utf8(&self) -> bool {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn is_always_start_anchored(&self) -> bool {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn start_state(&self, config: &start::Config) -> Result<StateID, StartError> {
        let anchored = config.get_anchored();
        let start = match config.get_look_behind() {
            None => Start::Text,
            Some(byte) => {
                if !self.quitset.is_empty() && self.quitset.contains(byte) {
                    return Err(StartError::quit(byte));
                }
                self.st.start_map.get(byte)
            }
        };
        self.st.start(anchored, start)
    }
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn universal_start_state(&self, mode: Anchored) -> Option<StateID> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn accelerator(&self, id: StateID) -> &[u8] {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn get_prefilter(&self) -> Option<&Prefilter> {}
}
impl Config {
    pub fn new() -> Config {}
    pub fn from_input_forward(input: &Input<'_>) -> Config {}
    pub fn from_input_reverse(input: &Input<'_>) -> Config {}
    pub fn look_behind(mut self, byte: Option<u8>) -> Config {}
    pub fn anchored(mut self, mode: Anchored) -> Config {}
    pub fn get_look_behind(&self) -> Option<u8> {
        self.look_behind
    }
    pub fn get_anchored(&self) -> Anchored {
        self.anchored
    }
}
impl<T: AsRef<[u32]>> StartTable<T> {
    fn write_to<E: Endian>(&self, mut dst: &mut [u8]) -> Result<usize, SerializeError> {}
    fn write_to_len(&self) -> usize {}
    fn validate(&self, dfa: &DFA<T>) -> Result<(), DeserializeError> {}
    fn as_ref(&self) -> StartTable<&'_ [u32]> {}
    #[cfg(feature = "alloc")]
    fn to_owned(&self) -> StartTable<alloc::vec::Vec<u32>> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn start(&self, anchored: Anchored, start: Start) -> Result<StateID, StartError> {
        let start_index = start.as_usize();
        let index = match anchored {
            Anchored::No => {
                if !self.kind.has_unanchored() {
                    return Err(StartError::unsupported_anchored(anchored));
                }
                start_index
            }
            Anchored::Yes => {
                if !self.kind.has_anchored() {
                    return Err(StartError::unsupported_anchored(anchored));
                }
                self.stride + start_index
            }
            Anchored::Pattern(pid) => {
                let len = match self.pattern_len {
                    None => return Err(StartError::unsupported_anchored(anchored)),
                    Some(len) => len,
                };
                if pid.as_usize() >= len {
                    return Ok(DEAD);
                }
                (2 * self.stride) + (self.stride * pid.as_usize()) + start_index
            }
        };
        Ok(self.table()[index])
    }
    fn iter(&self) -> StartStateIter<'_> {}
    fn table(&self) -> &[StateID] {}
    fn memory_usage(&self) -> usize {}
}
impl StartByteMap {
    pub(crate) fn new(lookm: &LookMatcher) -> StartByteMap {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn get(&self, byte: u8) -> Start {
        self.map[usize::from(byte)]
    }
    pub(crate) fn from_bytes(
        slice: &[u8],
    ) -> Result<(StartByteMap, usize), DeserializeError> {}
    pub(crate) fn write_to(&self, dst: &mut [u8]) -> Result<usize, SerializeError> {}
    pub(crate) fn write_to_len(&self) -> usize {}
}
impl ByteSet {
    pub(crate) fn empty() -> ByteSet {}
    pub(crate) fn add(&mut self, byte: u8) {}
    pub(crate) fn remove(&mut self, byte: u8) {}
    pub(crate) fn contains(&self, byte: u8) -> bool {
        let bucket = byte / 128;
        let bit = byte % 128;
        self.bits.0[usize::from(bucket)] & (1 << bit) > 0
    }
    pub(crate) fn contains_range(&self, start: u8, end: u8) -> bool {}
    pub(crate) fn iter(&self) -> ByteSetIter {}
    pub(crate) fn iter_ranges(&self) -> ByteSetRangeIter {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn is_empty(&self) -> bool {
        self.bits.0 == [0, 0]
    }
    pub(crate) fn from_bytes(
        slice: &[u8],
    ) -> Result<(ByteSet, usize), DeserializeError> {}
    pub(crate) fn write_to<E: crate::util::wire::Endian>(
        &self,
        dst: &mut [u8],
    ) -> Result<usize, SerializeError> {}
    pub(crate) fn write_to_len(&self) -> usize {}
}
impl StartError {
    pub(crate) fn quit(byte: u8) -> StartError {
        StartError::Quit { byte }
    }
    pub(crate) fn unsupported_anchored(mode: Anchored) -> StartError {}
}
