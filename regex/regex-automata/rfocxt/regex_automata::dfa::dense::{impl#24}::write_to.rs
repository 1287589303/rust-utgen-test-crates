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
pub struct PatternID(SmallIndex);
impl<T: AsRef<[u32]>> MatchStates<T> {
    fn write_to<E: Endian>(&self, mut dst: &mut [u8]) -> Result<usize, SerializeError> {
        let nwrite = self.write_to_len();
        if dst.len() < nwrite {
            return Err(SerializeError::buffer_too_small("match states"));
        }
        dst = &mut dst[..nwrite];
        E::write_u32(u32::try_from(self.len()).unwrap(), dst);
        dst = &mut dst[size_of::<u32>()..];
        for &pid in self.slices() {
            let n = wire::write_pattern_id::<E>(pid, &mut dst);
            dst = &mut dst[n..];
        }
        E::write_u32(u32::try_from(self.pattern_len).unwrap(), dst);
        dst = &mut dst[size_of::<u32>()..];
        E::write_u32(u32::try_from(self.pattern_ids().len()).unwrap(), dst);
        dst = &mut dst[size_of::<u32>()..];
        for &pid in self.pattern_ids() {
            let n = wire::write_pattern_id::<E>(pid, &mut dst);
            dst = &mut dst[n..];
        }
        Ok(nwrite)
    }
    fn write_to_len(&self) -> usize {
        size_of::<u32>() + (self.slices().len() * PatternID::SIZE) + size_of::<u32>()
            + size_of::<u32>() + (self.pattern_ids().len() * PatternID::SIZE)
    }
    fn validate(&self, dfa: &DFA<T>) -> Result<(), DeserializeError> {}
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
    fn slices(&self) -> &[PatternID] {
        wire::u32s_to_pattern_ids(self.slices.as_ref())
    }
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn len(&self) -> usize {
        assert_eq!(0, self.slices().len() % 2);
        self.slices().len() / 2
    }
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn pattern_ids(&self) -> &[PatternID] {
        wire::u32s_to_pattern_ids(self.pattern_ids.as_ref())
    }
    fn memory_usage(&self) -> usize {}
}
impl SerializeError {
    pub(crate) fn buffer_too_small(what: &'static str) -> SerializeError {
        SerializeError { what }
    }
}
pub(crate) fn write_pattern_id<E: Endian>(pid: PatternID, dst: &mut [u8]) -> usize {
    E::write_u32(pid.as_u32(), dst);
    PatternID::SIZE
}
