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
#[cfg(feature = "dfa-build")]
#[derive(Clone, Debug)]
pub struct BuildError {
    kind: BuildErrorKind,
}
#[derive(Clone)]
pub(crate) struct StartByteMap {
    map: [Start; 256],
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
#[derive(Clone, Debug)]
pub struct LookMatcher {
    lineterm: DebugByte,
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
pub enum StartKind {
    /// Support both anchored and unanchored searches.
    Both,
    /// Support only unanchored searches. Requesting an anchored search will
    /// panic.
    ///
    /// Note that even if an unanchored search is requested, the pattern itself
    /// may still be anchored. For example, `^abc` will only match `abc` at the
    /// start of a haystack. This will remain true, even if the regex engine
    /// only supported unanchored searches.
    Unanchored,
    /// Support only anchored searches. Requesting an unanchored search will
    /// panic.
    Anchored,
}
#[cfg(feature = "dfa-build")]
impl StartTable<Vec<u32>> {
    fn dead(
        kind: StartKind,
        lookm: &LookMatcher,
        pattern_len: Option<usize>,
    ) -> Result<StartTable<Vec<u32>>, BuildError> {
        if let Some(len) = pattern_len {
            assert!(len <= PatternID::LIMIT);
        }
        let stride = Start::len();
        let starts_len = stride.checked_mul(2).unwrap();
        let pattern_starts_len = match stride.checked_mul(pattern_len.unwrap_or(0)) {
            Some(x) => x,
            None => return Err(BuildError::too_many_start_states()),
        };
        let table_len = match starts_len.checked_add(pattern_starts_len) {
            Some(x) => x,
            None => return Err(BuildError::too_many_start_states()),
        };
        if let Err(_) = isize::try_from(table_len) {
            return Err(BuildError::too_many_start_states());
        }
        let table = vec![DEAD.as_u32(); table_len];
        let start_map = StartByteMap::new(lookm);
        Ok(StartTable {
            table,
            kind,
            start_map,
            stride,
            pattern_len,
            universal_start_unanchored: None,
            universal_start_anchored: None,
        })
    }
}
impl Start {
    pub(crate) fn from_usize(n: usize) -> Option<Start> {}
    pub(crate) fn len() -> usize {
        6
    }
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn as_u8(&self) -> u8 {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn as_usize(&self) -> usize {}
}
#[cfg(feature = "dfa-build")]
impl BuildError {
    fn kind(&self) -> &BuildErrorKind {}
    pub(crate) fn nfa(err: thompson::BuildError) -> BuildError {}
    pub(crate) fn unsupported_dfa_word_boundary_unicode() -> BuildError {}
    pub(crate) fn too_many_states() -> BuildError {}
    pub(crate) fn too_many_start_states() -> BuildError {
        BuildError {
            kind: BuildErrorKind::TooManyStartStates,
        }
    }
    pub(crate) fn too_many_match_pattern_ids() -> BuildError {}
    pub(crate) fn dfa_exceeded_size_limit(limit: usize) -> BuildError {}
    pub(crate) fn determinize_exceeded_size_limit(limit: usize) -> BuildError {}
}
impl StartByteMap {
    pub(crate) fn new(lookm: &LookMatcher) -> StartByteMap {
        let mut map = [Start::NonWordByte; 256];
        map[usize::from(b'\n')] = Start::LineLF;
        map[usize::from(b'\r')] = Start::LineCR;
        map[usize::from(b'_')] = Start::WordByte;
        let mut byte = b'0';
        while byte <= b'9' {
            map[usize::from(byte)] = Start::WordByte;
            byte += 1;
        }
        byte = b'A';
        while byte <= b'Z' {
            map[usize::from(byte)] = Start::WordByte;
            byte += 1;
        }
        byte = b'a';
        while byte <= b'z' {
            map[usize::from(byte)] = Start::WordByte;
            byte += 1;
        }
        let lineterm = lookm.get_line_terminator();
        if lineterm != b'\r' && lineterm != b'\n' {
            map[usize::from(lineterm)] = Start::CustomLineTerminator;
        }
        StartByteMap { map }
    }
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn get(&self, byte: u8) -> Start {}
    pub(crate) fn from_bytes(
        slice: &[u8],
    ) -> Result<(StartByteMap, usize), DeserializeError> {}
    pub(crate) fn write_to(&self, dst: &mut [u8]) -> Result<usize, SerializeError> {}
    pub(crate) fn write_to_len(&self) -> usize {}
}
