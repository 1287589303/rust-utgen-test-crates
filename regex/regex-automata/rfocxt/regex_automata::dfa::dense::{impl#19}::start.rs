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
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
#[derive(Clone)]
pub(crate) struct StartByteMap {
    map: [Start; 256],
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
    fn table(&self) -> &[StateID] {
        wire::u32s_to_state_ids(self.table.as_ref())
    }
    fn memory_usage(&self) -> usize {}
}
impl StartError {
    pub(crate) fn quit(byte: u8) -> StartError {}
    pub(crate) fn unsupported_anchored(mode: Anchored) -> StartError {
        StartError::UnsupportedAnchored {
            mode,
        }
    }
}
impl StartKind {
    pub(crate) fn from_bytes(
        slice: &[u8],
    ) -> Result<(StartKind, usize), DeserializeError> {}
    pub(crate) fn write_to<E: Endian>(
        &self,
        dst: &mut [u8],
    ) -> Result<usize, SerializeError> {}
    pub(crate) fn write_to_len(&self) -> usize {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn has_unanchored(&self) -> bool {
        matches!(* self, StartKind::Both | StartKind::Unanchored)
    }
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn has_anchored(&self) -> bool {
        matches!(* self, StartKind::Both | StartKind::Anchored)
    }
}
impl Start {
    pub(crate) fn from_usize(n: usize) -> Option<Start> {}
    pub(crate) fn len() -> usize {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn as_u8(&self) -> u8 {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn as_usize(&self) -> usize {
        usize::from(self.as_u8())
    }
}
