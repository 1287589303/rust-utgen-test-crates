#[cfg(feature = "dfa-build")]
use core::iter;
use core::{fmt, mem::size_of};
#[cfg(feature = "dfa-build")]
use alloc::{vec, vec::Vec};
#[cfg(feature = "dfa-build")]
use crate::dfa::dense::{self, BuildError};
use crate::{
    dfa::{
        automaton::{fmt_state_indicator, Automaton, StartError},
        dense::Flags, special::Special, StartKind, DEAD,
    },
    util::{
        alphabet::{ByteClasses, ByteSet},
        escape::DebugByte, int::{Pointer, Usize, U16, U32},
        prefilter::Prefilter, primitives::{PatternID, StateID},
        search::Anchored, start::{self, Start, StartByteMap},
        wire::{self, DeserializeError, Endian, SerializeError},
    },
};
const LABEL: &str = "rust-regex-automata-dfa-sparse";
const VERSION: u32 = 2;
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
#[derive(Clone)]
pub(crate) struct StartByteMap {
    map: [Start; 256],
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
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
impl<T: AsRef<[u8]>> StartTable<T> {
    fn write_to<E: Endian>(&self, mut dst: &mut [u8]) -> Result<usize, SerializeError> {}
    fn write_to_len(&self) -> usize {
        self.kind.write_to_len() + self.start_map.write_to_len() + size_of::<u32>()
            + size_of::<u32>() + size_of::<u32>() + size_of::<u32>() + self.table().len()
    }
    fn validate(&self, sp: &Special, seen: &Seen) -> Result<(), DeserializeError> {}
    fn as_ref(&self) -> StartTable<&'_ [u8]> {}
    #[cfg(feature = "alloc")]
    fn to_owned(&self) -> StartTable<alloc::vec::Vec<u8>> {}
    fn start(&self, anchored: Anchored, start: Start) -> Result<StateID, StartError> {}
    fn iter(&self) -> StartStateIter<'_, T> {}
    fn len(&self) -> usize {}
    fn table(&self) -> &[u8] {
        self.table.as_ref()
    }
    fn memory_usage(&self) -> usize {}
}
impl StartByteMap {
    pub(crate) fn new(lookm: &LookMatcher) -> StartByteMap {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn get(&self, byte: u8) -> Start {}
    pub(crate) fn from_bytes(
        slice: &[u8],
    ) -> Result<(StartByteMap, usize), DeserializeError> {}
    pub(crate) fn write_to(&self, dst: &mut [u8]) -> Result<usize, SerializeError> {}
    pub(crate) fn write_to_len(&self) -> usize {
        256
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
    pub(crate) fn write_to_len(&self) -> usize {
        size_of::<u32>()
    }
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn has_unanchored(&self) -> bool {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn has_anchored(&self) -> bool {}
}
