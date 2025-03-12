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
#[derive(Debug)]
pub struct SerializeError {
    /// The name of the thing that a buffer is too small for.
    ///
    /// Currently, the only kind of serialization error is one that is
    /// committed by a caller: providing a destination buffer that is too
    /// small to fit the serialized object. This makes sense conceptually,
    /// since every valid inhabitant of a type should be serializable.
    ///
    /// This is somewhat exposed in the public API of this crate. For example,
    /// the `to_bytes_{big,little}_endian` APIs return a `Vec<u8>` and are
    /// guaranteed to never panic or error. This is only possible because the
    /// implementation guarantees that it will allocate a `Vec<u8>` that is
    /// big enough.
    ///
    /// In summary, if a new serialization error kind needs to be added, then
    /// it will need careful consideration.
    what: &'static str,
}
#[derive(Clone)]
pub(crate) struct StartByteMap {
    map: [Start; 256],
}
struct StartStateIter<'a, T> {
    st: &'a StartTable<T>,
    i: usize,
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
impl<T: AsRef<[u8]>> StartTable<T> {
    fn write_to<E: Endian>(&self, mut dst: &mut [u8]) -> Result<usize, SerializeError> {
        let nwrite = self.write_to_len();
        if dst.len() < nwrite {
            return Err(SerializeError::buffer_too_small("sparse starting table ids"));
        }
        dst = &mut dst[..nwrite];
        let nw = self.kind.write_to::<E>(dst)?;
        dst = &mut dst[nw..];
        let nw = self.start_map.write_to(dst)?;
        dst = &mut dst[nw..];
        E::write_u32(u32::try_from(self.stride).unwrap(), dst);
        dst = &mut dst[size_of::<u32>()..];
        E::write_u32(
            u32::try_from(self.pattern_len.unwrap_or(0xFFFF_FFFF)).unwrap(),
            dst,
        );
        dst = &mut dst[size_of::<u32>()..];
        E::write_u32(
            self.universal_start_unanchored.map_or(u32::MAX, |sid| sid.as_u32()),
            dst,
        );
        dst = &mut dst[size_of::<u32>()..];
        E::write_u32(
            self.universal_start_anchored.map_or(u32::MAX, |sid| sid.as_u32()),
            dst,
        );
        dst = &mut dst[size_of::<u32>()..];
        for (sid, _, _) in self.iter() {
            E::write_u32(sid.as_u32(), dst);
            dst = &mut dst[StateID::SIZE..];
        }
        Ok(nwrite)
    }
    fn write_to_len(&self) -> usize {
        self.kind.write_to_len() + self.start_map.write_to_len() + size_of::<u32>()
            + size_of::<u32>() + size_of::<u32>() + size_of::<u32>() + self.table().len()
    }
    fn validate(&self, sp: &Special, seen: &Seen) -> Result<(), DeserializeError> {}
    fn as_ref(&self) -> StartTable<&'_ [u8]> {}
    #[cfg(feature = "alloc")]
    fn to_owned(&self) -> StartTable<alloc::vec::Vec<u8>> {}
    fn start(&self, anchored: Anchored, start: Start) -> Result<StateID, StartError> {}
    fn iter(&self) -> StartStateIter<'_, T> {
        StartStateIter { st: self, i: 0 }
    }
    fn len(&self) -> usize {}
    fn table(&self) -> &[u8] {}
    fn memory_usage(&self) -> usize {}
}
impl StartKind {
    pub(crate) fn from_bytes(
        slice: &[u8],
    ) -> Result<(StartKind, usize), DeserializeError> {}
    pub(crate) fn write_to<E: Endian>(
        &self,
        dst: &mut [u8],
    ) -> Result<usize, SerializeError> {
        let nwrite = self.write_to_len();
        if dst.len() < nwrite {
            return Err(SerializeError::buffer_too_small("start kind"));
        }
        let n = match *self {
            StartKind::Both => 0,
            StartKind::Unanchored => 1,
            StartKind::Anchored => 2,
        };
        E::write_u32(n, dst);
        Ok(nwrite)
    }
    pub(crate) fn write_to_len(&self) -> usize {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn has_unanchored(&self) -> bool {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn has_anchored(&self) -> bool {}
}
impl SerializeError {
    pub(crate) fn buffer_too_small(what: &'static str) -> SerializeError {
        SerializeError { what }
    }
}
impl StartByteMap {
    pub(crate) fn new(lookm: &LookMatcher) -> StartByteMap {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn get(&self, byte: u8) -> Start {}
    pub(crate) fn from_bytes(
        slice: &[u8],
    ) -> Result<(StartByteMap, usize), DeserializeError> {}
    pub(crate) fn write_to(&self, dst: &mut [u8]) -> Result<usize, SerializeError> {
        let nwrite = self.write_to_len();
        if dst.len() < nwrite {
            return Err(SerializeError::buffer_too_small("start byte map"));
        }
        for (i, &start) in self.map.iter().enumerate() {
            dst[i] = start.as_u8();
        }
        Ok(nwrite)
    }
    pub(crate) fn write_to_len(&self) -> usize {}
}
