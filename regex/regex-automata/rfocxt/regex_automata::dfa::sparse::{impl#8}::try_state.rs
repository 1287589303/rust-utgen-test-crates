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
struct Transitions<T> {
    /// The raw encoding of each state in this DFA.
    ///
    /// Each state has the following information:
    ///
    /// * A set of transitions to subsequent states. Transitions to the dead
    ///   state are omitted.
    /// * If the state can be accelerated, then any additional accelerator
    ///   information.
    /// * If the state is a match state, then the state contains all pattern
    ///   IDs that match when in that state.
    ///
    /// To decode a state, use Transitions::state.
    ///
    /// In practice, T is either Vec<u8> or &[u8].
    sparse: T,
    /// A set of equivalence classes, where a single equivalence class
    /// represents a set of bytes that never discriminate between a match
    /// and a non-match in the DFA. Each equivalence class corresponds to a
    /// single character in this DFA's alphabet, where the maximum number of
    /// characters is 257 (each possible value of a byte plus the special
    /// EOI transition). Consequently, the number of equivalence classes
    /// corresponds to the number of transitions for each DFA state. Note
    /// though that the *space* used by each DFA state in the transition table
    /// may be larger. The total space used by each DFA state is known as the
    /// stride and is documented above.
    ///
    /// The only time the number of equivalence classes is fewer than 257 is
    /// if the DFA's kind uses byte classes which is the default. Equivalence
    /// classes should generally only be disabled when debugging, so that
    /// the transitions themselves aren't obscured. Disabling them has no
    /// other benefit, since the equivalence class map is always used while
    /// searching. In the vast majority of cases, the number of equivalence
    /// classes is substantially smaller than 257, particularly when large
    /// Unicode classes aren't used.
    ///
    /// N.B. Equivalence classes aren't particularly useful in a sparse DFA
    /// in the current implementation, since equivalence classes generally tend
    /// to correspond to continuous ranges of bytes that map to the same
    /// transition. So in a sparse DFA, equivalence classes don't really lead
    /// to a space savings. In the future, it would be good to try and remove
    /// them from sparse DFAs entirely, but requires a bit of work since sparse
    /// DFAs are built from dense DFAs, which are in turn built on top of
    /// equivalence classes.
    classes: ByteClasses,
    /// The total number of states in this DFA. Note that a DFA always has at
    /// least one state---the dead state---even the empty DFA. In particular,
    /// the dead state always has ID 0 and is correspondingly always the first
    /// state. The dead state is never a match state.
    state_len: usize,
    /// The total number of unique patterns represented by these match states.
    pattern_len: usize,
}
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
pub struct DeserializeError(DeserializeErrorKind);
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
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
#[derive(Clone, Copy)]
pub struct ByteClasses([u8; 256]);
impl<T: AsRef<[u8]>> Transitions<T> {
    fn write_to<E: Endian>(&self, mut dst: &mut [u8]) -> Result<usize, SerializeError> {}
    fn write_to_len(&self) -> usize {}
    fn validate(&self, sp: &Special) -> Result<Seen, DeserializeError> {}
    fn as_ref(&self) -> Transitions<&'_ [u8]> {}
    #[cfg(feature = "alloc")]
    fn to_owned(&self) -> Transitions<alloc::vec::Vec<u8>> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn state(&self, id: StateID) -> State<'_> {}
    fn try_state(
        &self,
        sp: &Special,
        id: StateID,
    ) -> Result<State<'_>, DeserializeError> {
        if id.as_usize() > self.sparse().len() {
            return Err(
                DeserializeError::generic("invalid caller provided sparse state ID"),
            );
        }
        let mut state = &self.sparse()[id.as_usize()..];
        let (mut ntrans, _) = wire::try_read_u16_as_usize(
            state,
            "state transition length",
        )?;
        let is_match = ((1 << 15) & ntrans) != 0;
        ntrans &= !(1 << 15);
        state = &state[2..];
        if ntrans > 257 || ntrans == 0 {
            return Err(DeserializeError::generic("invalid transition length"));
        }
        if is_match && !sp.is_match_state(id) {
            return Err(
                DeserializeError::generic(
                    "state marked as match but not in match ID range",
                ),
            );
        } else if !is_match && sp.is_match_state(id) {
            return Err(
                DeserializeError::generic(
                    "state in match ID range but not marked as match state",
                ),
            );
        }
        let input_ranges_len = ntrans.checked_mul(2).unwrap();
        wire::check_slice_len(state, input_ranges_len, "sparse byte pairs")?;
        let (input_ranges, state) = state.split_at(input_ranges_len);
        for pair in input_ranges.chunks(2) {
            let (start, end) = (pair[0], pair[1]);
            if start > end {
                return Err(DeserializeError::generic("invalid input range"));
            }
        }
        let next_len = ntrans
            .checked_mul(self.id_len())
            .expect("state size * #trans should always fit in a usize");
        wire::check_slice_len(state, next_len, "sparse trans state IDs")?;
        let (next, state) = state.split_at(next_len);
        for idbytes in next.chunks(self.id_len()) {
            let (id, _) = wire::read_state_id(idbytes, "sparse state ID in try_state")?;
            wire::check_slice_len(
                self.sparse(),
                id.as_usize(),
                "invalid sparse state ID",
            )?;
        }
        let (pattern_ids, state) = if is_match {
            let (npats, nr) = wire::try_read_u32_as_usize(state, "pattern ID length")?;
            let state = &state[nr..];
            if npats == 0 {
                return Err(
                    DeserializeError::generic(
                        "state marked as a match, but pattern length is zero",
                    ),
                );
            }
            let pattern_ids_len = wire::mul(npats, 4, "sparse pattern ID byte length")?;
            wire::check_slice_len(state, pattern_ids_len, "sparse pattern IDs")?;
            let (pattern_ids, state) = state.split_at(pattern_ids_len);
            for patbytes in pattern_ids.chunks(PatternID::SIZE) {
                wire::read_pattern_id(patbytes, "sparse pattern ID in try_state")?;
            }
            (pattern_ids, state)
        } else {
            (&[][..], state)
        };
        if is_match && pattern_ids.is_empty() {
            return Err(
                DeserializeError::generic(
                    "state marked as a match, but has no pattern IDs",
                ),
            );
        }
        if sp.is_match_state(id) && pattern_ids.is_empty() {
            return Err(
                DeserializeError::generic(
                    "state marked special as a match, but has no pattern IDs",
                ),
            );
        }
        if sp.is_match_state(id) != is_match {
            return Err(
                DeserializeError::generic(
                    "whether state is a match or not is inconsistent",
                ),
            );
        }
        if state.is_empty() {
            return Err(DeserializeError::generic("no accelerator length"));
        }
        let (accel_len, state) = (usize::from(state[0]), &state[1..]);
        if accel_len > 3 {
            return Err(DeserializeError::generic("sparse invalid accelerator length"));
        } else if accel_len == 0 && sp.is_accel_state(id) {
            return Err(
                DeserializeError::generic(
                    "got no accelerators in state, but in accelerator ID range",
                ),
            );
        } else if accel_len > 0 && !sp.is_accel_state(id) {
            return Err(
                DeserializeError::generic(
                    "state in accelerator ID range, but has no accelerators",
                ),
            );
        }
        wire::check_slice_len(state, accel_len, "sparse corrupt accelerator length")?;
        let (accel, _) = (&state[..accel_len], &state[accel_len..]);
        let state = State {
            id,
            is_match,
            ntrans,
            input_ranges,
            next,
            pattern_ids,
            accel,
        };
        if sp.is_quit_state(state.next_at(state.ntrans - 1)) {
            return Err(
                DeserializeError::generic(
                    "state with EOI transition to quit state is illegal",
                ),
            );
        }
        Ok(state)
    }
    fn states(&self) -> StateIter<'_, T> {}
    fn sparse(&self) -> &[u8] {
        self.sparse.as_ref()
    }
    fn id_len(&self) -> usize {
        StateID::SIZE
    }
    fn memory_usage(&self) -> usize {}
}
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
    fn pattern_id(&self, match_index: usize) -> PatternID {}
    fn pattern_len(&self) -> usize {}
    fn accelerator(&self) -> &'a [u8] {}
    fn write_to<E: Endian>(&self, mut dst: &mut [u8]) -> Result<usize, SerializeError> {}
    fn write_to_len(&self) -> usize {}
}
impl DeserializeError {
    pub(crate) fn generic(msg: &'static str) -> DeserializeError {
        DeserializeError(DeserializeErrorKind::Generic {
            msg,
        })
    }
    pub(crate) fn buffer_too_small(what: &'static str) -> DeserializeError {}
    fn invalid_usize(what: &'static str) -> DeserializeError {}
    fn version_mismatch(expected: u32, found: u32) -> DeserializeError {}
    fn endian_mismatch(expected: u32, found: u32) -> DeserializeError {}
    fn alignment_mismatch(alignment: usize, address: usize) -> DeserializeError {}
    fn label_mismatch(expected: &'static str) -> DeserializeError {}
    fn arithmetic_overflow(what: &'static str) -> DeserializeError {}
    fn pattern_id_error(err: PatternIDError, what: &'static str) -> DeserializeError {}
    pub(crate) fn state_id_error(
        err: StateIDError,
        what: &'static str,
    ) -> DeserializeError {}
}
impl Special {
    #[cfg(feature = "dfa-build")]
    pub(crate) fn new() -> Special {}
    #[cfg(feature = "dfa-build")]
    pub(crate) fn remap(&self, map: impl Fn(StateID) -> StateID) -> Special {}
    pub(crate) fn from_bytes(
        mut slice: &[u8],
    ) -> Result<(Special, usize), DeserializeError> {}
    pub(crate) fn validate(&self) -> Result<(), DeserializeError> {}
    pub(crate) fn validate_state_len(
        &self,
        len: usize,
        stride2: usize,
    ) -> Result<(), DeserializeError> {}
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
    pub(crate) fn is_quit_state(&self, id: StateID) -> bool {
        !self.is_dead_state(id) && self.quit_id == id
    }
    #[inline]
    pub(crate) fn is_match_state(&self, id: StateID) -> bool {
        !self.is_dead_state(id) && self.min_match <= id && id <= self.max_match
    }
    #[inline]
    pub(crate) fn is_accel_state(&self, id: StateID) -> bool {
        !self.is_dead_state(id) && self.min_accel <= id && id <= self.max_accel
    }
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
pub(crate) fn try_read_u16_as_usize(
    slice: &[u8],
    what: &'static str,
) -> Result<(usize, usize), DeserializeError> {
    try_read_u16(slice, what)
        .and_then(|(n, nr)| {
            usize::try_from(n)
                .map(|n| (n, nr))
                .map_err(|_| DeserializeError::invalid_usize(what))
        })
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
pub(crate) fn read_pattern_id(
    slice: &[u8],
    what: &'static str,
) -> Result<(PatternID, usize), DeserializeError> {
    let bytes: [u8; PatternID::SIZE] = slice[..PatternID::SIZE].try_into().unwrap();
    let pid = PatternID::from_ne_bytes(bytes)
        .map_err(|err| DeserializeError::pattern_id_error(err, what))?;
    Ok((pid, PatternID::SIZE))
}
pub(crate) fn read_state_id(
    slice: &[u8],
    what: &'static str,
) -> Result<(StateID, usize), DeserializeError> {
    let bytes: [u8; StateID::SIZE] = slice[..StateID::SIZE].try_into().unwrap();
    let sid = StateID::from_ne_bytes(bytes)
        .map_err(|err| DeserializeError::state_id_error(err, what))?;
    Ok((sid, StateID::SIZE))
}
