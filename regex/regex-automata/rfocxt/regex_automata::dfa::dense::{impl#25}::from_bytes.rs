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
#[derive(Debug)]
pub struct DeserializeError(DeserializeErrorKind);
impl Flags {
    #[cfg(feature = "dfa-build")]
    fn from_nfa(nfa: &thompson::NFA) -> Flags {}
    pub(crate) fn from_bytes(slice: &[u8]) -> Result<(Flags, usize), DeserializeError> {
        let (bits, nread) = wire::try_read_u32(slice, "flag bitset")?;
        let flags = Flags {
            has_empty: bits & (1 << 0) != 0,
            is_utf8: bits & (1 << 1) != 0,
            is_always_start_anchored: bits & (1 << 2) != 0,
        };
        Ok((flags, nread))
    }
    pub(crate) fn write_to<E: Endian>(
        &self,
        dst: &mut [u8],
    ) -> Result<usize, SerializeError> {}
    pub(crate) fn write_to_len(&self) -> usize {}
}
pub(crate) fn try_read_u32(
    slice: &[u8],
    what: &'static str,
) -> Result<(u32, usize), DeserializeError> {
    check_slice_len(slice, size_of::<u32>(), what)?;
    Ok((read_u32(slice), size_of::<u32>()))
}
