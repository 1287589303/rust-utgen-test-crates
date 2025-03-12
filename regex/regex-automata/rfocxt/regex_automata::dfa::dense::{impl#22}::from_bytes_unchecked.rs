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
pub struct DeserializeError(DeserializeErrorKind);
impl<'a> MatchStates<&'a [u32]> {
    unsafe fn from_bytes_unchecked(
        mut slice: &'a [u8],
    ) -> Result<(MatchStates<&'a [u32]>, usize), DeserializeError> {
        let slice_start = slice.as_ptr().as_usize();
        let (state_len, nr) = wire::try_read_u32_as_usize(slice, "match state length")?;
        slice = &slice[nr..];
        let pair_len = wire::mul(2, state_len, "match state offset pairs")?;
        let slices_bytes_len = wire::mul(
            pair_len,
            PatternID::SIZE,
            "match state slice offset byte length",
        )?;
        wire::check_slice_len(slice, slices_bytes_len, "match state slices")?;
        wire::check_alignment::<PatternID>(slice)?;
        let slices_bytes = &slice[..slices_bytes_len];
        slice = &slice[slices_bytes_len..];
        let slices = core::slice::from_raw_parts(
            slices_bytes.as_ptr().cast::<u32>(),
            pair_len,
        );
        let (pattern_len, nr) = wire::try_read_u32_as_usize(slice, "pattern length")?;
        slice = &slice[nr..];
        let (idlen, nr) = wire::try_read_u32_as_usize(slice, "pattern ID length")?;
        slice = &slice[nr..];
        let pattern_ids_len = wire::mul(
            idlen,
            PatternID::SIZE,
            "pattern ID byte length",
        )?;
        wire::check_slice_len(slice, pattern_ids_len, "match pattern IDs")?;
        wire::check_alignment::<PatternID>(slice)?;
        let pattern_ids_bytes = &slice[..pattern_ids_len];
        slice = &slice[pattern_ids_len..];
        let pattern_ids = core::slice::from_raw_parts(
            pattern_ids_bytes.as_ptr().cast::<u32>(),
            idlen,
        );
        let ms = MatchStates {
            slices,
            pattern_ids,
            pattern_len,
        };
        Ok((ms, slice.as_ptr().as_usize() - slice_start))
    }
}
pub(crate) fn check_alignment<T>(slice: &[u8]) -> Result<(), DeserializeError> {
    let alignment = core::mem::align_of::<T>();
    let address = slice.as_ptr().as_usize();
    if address % alignment == 0 {
        return Ok(());
    }
    Err(DeserializeError::alignment_mismatch(alignment, address))
}
pub(crate) fn mul(
    a: usize,
    b: usize,
    what: &'static str,
) -> Result<usize, DeserializeError> {
    match a.checked_mul(b) {
        Some(c) => Ok(c),
        None => Err(DeserializeError::arithmetic_overflow(what)),
    }
}
pub(crate) fn try_read_u32_as_usize(
    slice: &[u8],
    what: &'static str,
) -> Result<(usize, usize), DeserializeError> {
    try_read_u32(slice, what)
        .and_then(|(n, nr)| {
            usize::try_from(n)
                .map(|n| (n, nr))
                .map_err(|_| DeserializeError::invalid_usize(what))
        })
}
pub(crate) fn check_slice_len<T>(
    slice: &[T],
    at_least_len: usize,
    what: &'static str,
) -> Result<(), DeserializeError> {
    if slice.len() < at_least_len {
        return Err(DeserializeError::buffer_too_small(what));
    }
    Ok(())
}
