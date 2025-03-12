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
pub(crate) trait U32 {
    fn as_usize(self) -> usize;
    fn low_u8(self) -> u8;
    fn low_u16(self) -> u16;
    fn high_u16(self) -> u16;
}
pub(crate) trait U16 {
    fn as_usize(self) -> usize;
    fn low_u8(self) -> u8;
    fn high_u8(self) -> u8;
}
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
#[derive(Clone, Copy)]
pub struct ByteClasses([u8; 256]);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
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
impl<T: AsRef<[u8]>> Transitions<T> {
    fn write_to<E: Endian>(&self, mut dst: &mut [u8]) -> Result<usize, SerializeError> {}
    fn write_to_len(&self) -> usize {}
    fn validate(&self, sp: &Special) -> Result<Seen, DeserializeError> {}
    fn as_ref(&self) -> Transitions<&'_ [u8]> {}
    #[cfg(feature = "alloc")]
    fn to_owned(&self) -> Transitions<alloc::vec::Vec<u8>> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn state(&self, id: StateID) -> State<'_> {
        let mut state = &self.sparse()[id.as_usize()..];
        let mut ntrans = wire::read_u16(&state).as_usize();
        let is_match = (1 << 15) & ntrans != 0;
        ntrans &= !(1 << 15);
        state = &state[2..];
        let (input_ranges, state) = state.split_at(ntrans * 2);
        let (next, state) = state.split_at(ntrans * StateID::SIZE);
        let (pattern_ids, state) = if is_match {
            let npats = wire::read_u32(&state).as_usize();
            state[4..].split_at(npats * 4)
        } else {
            (&[][..], state)
        };
        let accel_len = usize::from(state[0]);
        let accel = &state[1..accel_len + 1];
        State {
            id,
            is_match,
            ntrans,
            input_ranges,
            next,
            pattern_ids,
            accel,
        }
    }
    fn try_state(
        &self,
        sp: &Special,
        id: StateID,
    ) -> Result<State<'_>, DeserializeError> {}
    fn states(&self) -> StateIter<'_, T> {}
    fn sparse(&self) -> &[u8] {
        self.sparse.as_ref()
    }
    fn id_len(&self) -> usize {}
    fn memory_usage(&self) -> usize {}
}
#[cfg_attr(feature = "perf-inline", inline(always))]
pub(crate) fn read_u32(slice: &[u8]) -> u32 {
    let bytes: [u8; 4] = slice[..size_of::<u32>()].try_into().unwrap();
    u32::from_ne_bytes(bytes)
}
#[cfg_attr(feature = "perf-inline", inline(always))]
pub(crate) fn read_u16(slice: &[u8]) -> u16 {
    let bytes: [u8; 2] = slice[..size_of::<u16>()].try_into().unwrap();
    u16::from_ne_bytes(bytes)
}
