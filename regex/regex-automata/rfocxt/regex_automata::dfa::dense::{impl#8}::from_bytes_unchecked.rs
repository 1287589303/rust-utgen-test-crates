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
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct ByteSet {
    bits: BitSet,
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
impl<'a> DFA<&'a [u32]> {
    pub fn from_bytes(
        slice: &'a [u8],
    ) -> Result<(DFA<&'a [u32]>, usize), DeserializeError> {}
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
impl<T: AsRef<[u32]>> TransitionTable<T> {
    fn write_to<E: Endian>(&self, mut dst: &mut [u8]) -> Result<usize, SerializeError> {}
    fn write_to_len(&self) -> usize {}
    fn validate(&self, dfa: &DFA<T>) -> Result<(), DeserializeError> {}
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
    fn len(&self) -> usize {
        self.table().len() >> self.stride2
    }
    fn stride(&self) -> usize {}
    fn alphabet_len(&self) -> usize {}
    fn is_valid(&self, id: StateID) -> bool {}
    fn memory_usage(&self) -> usize {}
}
impl ByteSet {
    pub(crate) fn empty() -> ByteSet {}
    pub(crate) fn add(&mut self, byte: u8) {}
    pub(crate) fn remove(&mut self, byte: u8) {}
    pub(crate) fn contains(&self, byte: u8) -> bool {}
    pub(crate) fn contains_range(&self, start: u8, end: u8) -> bool {}
    pub(crate) fn iter(&self) -> ByteSetIter {}
    pub(crate) fn iter_ranges(&self) -> ByteSetRangeIter {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn is_empty(&self) -> bool {}
    pub(crate) fn from_bytes(
        slice: &[u8],
    ) -> Result<(ByteSet, usize), DeserializeError> {
        use core::mem::size_of;
        wire::check_slice_len(slice, 2 * size_of::<u128>(), "byte set")?;
        let mut nread = 0;
        let (low, nr) = wire::try_read_u128(slice, "byte set low bucket")?;
        nread += nr;
        let (high, nr) = wire::try_read_u128(slice, "byte set high bucket")?;
        nread += nr;
        Ok((
            ByteSet {
                bits: BitSet([low, high]),
            },
            nread,
        ))
    }
    pub(crate) fn write_to<E: crate::util::wire::Endian>(
        &self,
        dst: &mut [u8],
    ) -> Result<usize, SerializeError> {}
    pub(crate) fn write_to_len(&self) -> usize {}
}
impl<'a> Accels<&'a [AccelTy]> {
    pub fn from_bytes_unchecked(
        mut slice: &'a [u8],
    ) -> Result<(Accels<&'a [AccelTy]>, usize), DeserializeError> {
        let slice_start = slice.as_ptr().as_usize();
        let (accel_len, _) = wire::try_read_u32_as_usize(slice, "accelerators length")?;
        let accel_tys_len = wire::add(
            wire::mul(accel_len, 2, "total number of accelerator accel_tys")?,
            1,
            "total number of accel_tys",
        )?;
        let accel_tys_bytes_len = wire::mul(
            ACCEL_TY_SIZE,
            accel_tys_len,
            "total number of bytes in accelerators",
        )?;
        wire::check_slice_len(slice, accel_tys_bytes_len, "accelerators")?;
        wire::check_alignment::<AccelTy>(slice)?;
        let accel_tys = &slice[..accel_tys_bytes_len];
        slice = &slice[accel_tys_bytes_len..];
        let accels = unsafe {
            core::slice::from_raw_parts(
                accel_tys.as_ptr().cast::<AccelTy>(),
                accel_tys_len,
            )
        };
        Ok((Accels { accels }, slice.as_ptr().as_usize() - slice_start))
    }
}
impl<'a> StartTable<&'a [u32]> {
    unsafe fn from_bytes_unchecked(
        mut slice: &'a [u8],
    ) -> Result<(StartTable<&'a [u32]>, usize), DeserializeError> {
        let slice_start = slice.as_ptr().as_usize();
        let (kind, nr) = StartKind::from_bytes(slice)?;
        slice = &slice[nr..];
        let (start_map, nr) = StartByteMap::from_bytes(slice)?;
        slice = &slice[nr..];
        let (stride, nr) = wire::try_read_u32_as_usize(slice, "start table stride")?;
        slice = &slice[nr..];
        if stride != Start::len() {
            return Err(DeserializeError::generic("invalid starting table stride"));
        }
        let (maybe_pattern_len, nr) = wire::try_read_u32_as_usize(
            slice,
            "start table patterns",
        )?;
        slice = &slice[nr..];
        let pattern_len = if maybe_pattern_len.as_u32() == u32::MAX {
            None
        } else {
            Some(maybe_pattern_len)
        };
        if pattern_len.map_or(false, |len| len > PatternID::LIMIT) {
            return Err(DeserializeError::generic("invalid number of patterns"));
        }
        let (universal_unanchored, nr) = wire::try_read_u32(
            slice,
            "universal unanchored start",
        )?;
        slice = &slice[nr..];
        let universal_start_unanchored = if universal_unanchored == u32::MAX {
            None
        } else {
            Some(
                StateID::try_from(universal_unanchored)
                    .map_err(|e| {
                        DeserializeError::state_id_error(e, "universal unanchored start")
                    })?,
            )
        };
        let (universal_anchored, nr) = wire::try_read_u32(
            slice,
            "universal anchored start",
        )?;
        slice = &slice[nr..];
        let universal_start_anchored = if universal_anchored == u32::MAX {
            None
        } else {
            Some(
                StateID::try_from(universal_anchored)
                    .map_err(|e| {
                        DeserializeError::state_id_error(e, "universal anchored start")
                    })?,
            )
        };
        let pattern_table_size = wire::mul(
            stride,
            pattern_len.unwrap_or(0),
            "invalid pattern length",
        )?;
        let start_state_len = wire::add(
            wire::mul(2, stride, "start state stride too big")?,
            pattern_table_size,
            "invalid 'any' pattern starts size",
        )?;
        let table_bytes_len = wire::mul(
            start_state_len,
            StateID::SIZE,
            "pattern table bytes length",
        )?;
        wire::check_slice_len(slice, table_bytes_len, "start ID table")?;
        wire::check_alignment::<StateID>(slice)?;
        let table_bytes = &slice[..table_bytes_len];
        slice = &slice[table_bytes_len..];
        let table = core::slice::from_raw_parts(
            table_bytes.as_ptr().cast::<u32>(),
            start_state_len,
        );
        let st = StartTable {
            table,
            kind,
            start_map,
            stride,
            pattern_len,
            universal_start_unanchored,
            universal_start_anchored,
        };
        Ok((st, slice.as_ptr().as_usize() - slice_start))
    }
}
impl Special {
    #[cfg(feature = "dfa-build")]
    pub(crate) fn new() -> Special {}
    #[cfg(feature = "dfa-build")]
    pub(crate) fn remap(&self, map: impl Fn(StateID) -> StateID) -> Special {}
    pub(crate) fn from_bytes(
        mut slice: &[u8],
    ) -> Result<(Special, usize), DeserializeError> {
        wire::check_slice_len(slice, 8 * StateID::SIZE, "special states")?;
        let mut nread = 0;
        let mut read_id = |what| -> Result<StateID, DeserializeError> {
            let (id, nr) = wire::try_read_state_id(slice, what)?;
            nread += nr;
            slice = &slice[StateID::SIZE..];
            Ok(id)
        };
        let max = read_id("special max id")?;
        let quit_id = read_id("special quit id")?;
        let min_match = read_id("special min match id")?;
        let max_match = read_id("special max match id")?;
        let min_accel = read_id("special min accel id")?;
        let max_accel = read_id("special max accel id")?;
        let min_start = read_id("special min start id")?;
        let max_start = read_id("special max start id")?;
        let special = Special {
            max,
            quit_id,
            min_match,
            max_match,
            min_accel,
            max_accel,
            min_start,
            max_start,
        };
        special.validate()?;
        assert_eq!(nread, special.write_to_len());
        Ok((special, nread))
    }
    pub(crate) fn validate(&self) -> Result<(), DeserializeError> {}
    pub(crate) fn validate_state_len(
        &self,
        len: usize,
        stride2: usize,
    ) -> Result<(), DeserializeError> {
        if (self.max.as_usize() >> stride2) >= len {
            err!("max should not be greater than or equal to state length");
        }
        Ok(())
    }
    pub(crate) fn write_to<E: Endian>(
        &self,
        dst: &mut [u8],
    ) -> Result<usize, SerializeError> {}
    pub(crate) fn write_to_len(&self) -> usize {}
    #[cfg(feature = "dfa-build")]
    pub(crate) fn set_max(&mut self) {}
    #[cfg(feature = "dfa-build")]
    pub(crate) fn set_no_special_start_states(&mut self) {}
    #[inline]
    pub(crate) fn is_special_state(&self, id: StateID) -> bool {}
    #[inline]
    pub(crate) fn is_dead_state(&self, id: StateID) -> bool {}
    #[inline]
    pub(crate) fn is_quit_state(&self, id: StateID) -> bool {}
    #[inline]
    pub(crate) fn is_match_state(&self, id: StateID) -> bool {}
    #[inline]
    pub(crate) fn is_accel_state(&self, id: StateID) -> bool {}
    #[inline]
    pub(crate) fn is_start_state(&self, id: StateID) -> bool {}
    #[inline]
    pub(crate) fn match_len(&self, stride: usize) -> usize {}
    #[inline]
    pub(crate) fn matches(&self) -> bool {}
    #[cfg(feature = "dfa-build")]
    pub(crate) fn accel_len(&self, stride: usize) -> usize {}
    #[inline]
    pub(crate) fn accels(&self) -> bool {}
    #[inline]
    pub(crate) fn starts(&self) -> bool {}
}
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
impl<'a> TransitionTable<&'a [u32]> {
    unsafe fn from_bytes_unchecked(
        mut slice: &'a [u8],
    ) -> Result<(TransitionTable<&'a [u32]>, usize), DeserializeError> {
        let slice_start = slice.as_ptr().as_usize();
        let (state_len, nr) = wire::try_read_u32_as_usize(slice, "state length")?;
        slice = &slice[nr..];
        let (stride2, nr) = wire::try_read_u32_as_usize(slice, "stride2")?;
        slice = &slice[nr..];
        let (classes, nr) = ByteClasses::from_bytes(slice)?;
        slice = &slice[nr..];
        if stride2 > 9 {
            return Err(
                DeserializeError::generic("dense DFA has invalid stride2 (too big)"),
            );
        }
        if stride2 < 1 {
            return Err(
                DeserializeError::generic("dense DFA has invalid stride2 (too small)"),
            );
        }
        let stride = 1usize.checked_shl(u32::try_from(stride2).unwrap()).unwrap();
        if classes.alphabet_len() > stride {
            return Err(
                DeserializeError::generic(
                    "alphabet size cannot be bigger than transition table stride",
                ),
            );
        }
        let trans_len = wire::shl(state_len, stride2, "dense table transition length")?;
        let table_bytes_len = wire::mul(
            trans_len,
            StateID::SIZE,
            "dense table state byte length",
        )?;
        wire::check_slice_len(slice, table_bytes_len, "transition table")?;
        wire::check_alignment::<StateID>(slice)?;
        let table_bytes = &slice[..table_bytes_len];
        slice = &slice[table_bytes_len..];
        let table = core::slice::from_raw_parts(
            table_bytes.as_ptr().cast::<u32>(),
            trans_len,
        );
        let tt = TransitionTable {
            table,
            classes,
            stride2,
        };
        Ok((tt, slice.as_ptr().as_usize() - slice_start))
    }
}
pub(crate) fn skip_initial_padding(slice: &[u8]) -> usize {
    let mut nread = 0;
    while nread < 7 && nread < slice.len() && slice[nread] == 0 {
        nread += 1;
    }
    nread
}
pub(crate) fn read_version(
    slice: &[u8],
    expected_version: u32,
) -> Result<usize, DeserializeError> {
    let (n, nr) = try_read_u32(slice, "version")?;
    assert_eq!(nr, write_version_len());
    if n != expected_version {
        return Err(DeserializeError::version_mismatch(expected_version, n));
    }
    Ok(nr)
}
pub(crate) fn read_label(
    slice: &[u8],
    expected_label: &'static str,
) -> Result<usize, DeserializeError> {
    let first_nul = slice[..cmp::min(slice.len(), 256)].iter().position(|&b| b == 0);
    let first_nul = match first_nul {
        Some(first_nul) => first_nul,
        None => {
            return Err(
                DeserializeError::generic(
                    "could not find NUL terminated label \
                 at start of serialized object",
                ),
            );
        }
    };
    let len = first_nul + padding_len(first_nul);
    if slice.len() < len {
        return Err(
            DeserializeError::generic(
                "could not find properly sized label at start of serialized object",
            ),
        );
    }
    if expected_label.as_bytes() != &slice[..first_nul] {
        return Err(DeserializeError::label_mismatch(expected_label));
    }
    Ok(len)
}
pub(crate) fn check_alignment<T>(slice: &[u8]) -> Result<(), DeserializeError> {
    let alignment = core::mem::align_of::<T>();
    let address = slice.as_ptr().as_usize();
    if address % alignment == 0 {
        return Ok(());
    }
    Err(DeserializeError::alignment_mismatch(alignment, address))
}
pub(crate) fn try_read_u32(
    slice: &[u8],
    what: &'static str,
) -> Result<(u32, usize), DeserializeError> {
    check_slice_len(slice, size_of::<u32>(), what)?;
    Ok((read_u32(slice), size_of::<u32>()))
}
pub(crate) fn read_endianness_check(slice: &[u8]) -> Result<usize, DeserializeError> {
    let (n, nr) = try_read_u32(slice, "endianness check")?;
    assert_eq!(nr, write_endianness_check_len());
    if n != 0xFEFF {
        return Err(DeserializeError::endian_mismatch(0xFEFF, n));
    }
    Ok(nr)
}
