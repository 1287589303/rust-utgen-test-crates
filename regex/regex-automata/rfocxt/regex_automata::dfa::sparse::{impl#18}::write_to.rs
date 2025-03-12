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
struct State<'a> {
    /// The identifier of this state.
    id: StateID,
    /// Whether this is a match state or not.
    is_match: bool,
    /// The number of transitions in this state.
    ntrans: usize,
    /// Pairs of input ranges, where there is one pair for each transition.
    /// Each pair specifies an inclusive start and end byte range for the
    /// corresponding transition.
    input_ranges: &'a [u8],
    /// Transitions to the next state. This slice contains native endian
    /// encoded state identifiers, with `S` as the representation. Thus, there
    /// are `ntrans * size_of::<S>()` bytes in this slice.
    next: &'a [u8],
    /// If this is a match state, then this contains the pattern IDs that match
    /// when the DFA is in this state.
    ///
    /// This is a contiguous sequence of 32-bit native endian encoded integers.
    pattern_ids: &'a [u8],
    /// An accelerator for this state, if present. If this state has no
    /// accelerator, then this is an empty slice. When non-empty, this slice
    /// has length at most 3 and corresponds to the exhaustive set of bytes
    /// that must be seen in order to transition out of this state.
    accel: &'a [u8],
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
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
impl<'a> State<'a> {
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn next(&self, input: u8) -> StateID {}
    fn next_eoi(&self) -> StateID {}
    fn id(&self) -> StateID {}
    fn range(&self, i: usize) -> (u8, u8) {}
    fn next_at(&self, i: usize) -> StateID {
        let start = i * StateID::SIZE;
        let end = start + StateID::SIZE;
        let bytes = self.next[start..end].try_into().unwrap();
        StateID::from_ne_bytes_unchecked(bytes)
    }
    fn pattern_id(&self, match_index: usize) -> PatternID {
        let start = match_index * PatternID::SIZE;
        wire::read_pattern_id_unchecked(&self.pattern_ids[start..]).0
    }
    fn pattern_len(&self) -> usize {
        assert_eq!(0, self.pattern_ids.len() % 4);
        self.pattern_ids.len() / 4
    }
    fn accelerator(&self) -> &'a [u8] {}
    fn write_to<E: Endian>(&self, mut dst: &mut [u8]) -> Result<usize, SerializeError> {
        let nwrite = self.write_to_len();
        if dst.len() < nwrite {
            return Err(SerializeError::buffer_too_small("sparse state transitions"));
        }
        let ntrans = if self.is_match { self.ntrans | (1 << 15) } else { self.ntrans };
        E::write_u16(u16::try_from(ntrans).unwrap(), dst);
        dst = &mut dst[size_of::<u16>()..];
        dst[..self.input_ranges.len()].copy_from_slice(self.input_ranges);
        dst = &mut dst[self.input_ranges.len()..];
        for i in 0..self.ntrans {
            E::write_u32(self.next_at(i).as_u32(), dst);
            dst = &mut dst[StateID::SIZE..];
        }
        if self.is_match {
            E::write_u32(u32::try_from(self.pattern_len()).unwrap(), dst);
            dst = &mut dst[size_of::<u32>()..];
            for i in 0..self.pattern_len() {
                let pid = self.pattern_id(i);
                E::write_u32(pid.as_u32(), dst);
                dst = &mut dst[PatternID::SIZE..];
            }
        }
        dst[0] = u8::try_from(self.accel.len()).unwrap();
        dst[1..][..self.accel.len()].copy_from_slice(self.accel);
        Ok(nwrite)
    }
    fn write_to_len(&self) -> usize {
        let mut len = 2 + (self.ntrans * 2) + (self.ntrans * StateID::SIZE)
            + (1 + self.accel.len());
        if self.is_match {
            len += size_of::<u32>() + self.pattern_ids.len();
        }
        len
    }
}
impl SerializeError {
    pub(crate) fn buffer_too_small(what: &'static str) -> SerializeError {
        SerializeError { what }
    }
}
