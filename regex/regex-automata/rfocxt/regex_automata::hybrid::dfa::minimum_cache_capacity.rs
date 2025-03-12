#[cfg(feature = "std")]
type StateMap = std::collections::HashMap<State, LazyStateID>;
#[cfg(not(feature = "std"))]
type StateMap = alloc::collections::BTreeMap<State, LazyStateID>;
use core::{iter, mem::size_of};
use alloc::vec::Vec;
use crate::{
    hybrid::{
        error::{BuildError, CacheError, StartError},
        id::{LazyStateID, LazyStateIDError},
        search,
    },
    nfa::thompson,
    util::{
        alphabet::{self, ByteClasses, ByteSet},
        determinize::{self, State, StateBuilderEmpty, StateBuilderNFA},
        empty, prefilter::Prefilter, primitives::{PatternID, StateID as NFAStateID},
        search::{Anchored, HalfMatch, Input, MatchError, MatchKind, PatternSet},
        sparse_set::SparseSets, start::{self, Start, StartByteMap},
    },
};
const MIN_STATES: usize = SENTINEL_STATES + 2;
const SENTINEL_STATES: usize = 3;
#[derive(Clone)]
pub struct NFA(Arc<Inner>);
#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub(crate) struct State(Arc<[u8]>);
#[derive(Clone, Copy)]
pub struct ByteClasses([u8; 256]);
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
#[derive(Clone, Eq, PartialEq)]
pub enum State {
    /// A state with a single transition that can only be taken if the current
    /// input symbol is in a particular range of bytes.
    ByteRange {
        /// The transition from this state to the next.
        trans: Transition,
    },
    /// A state with possibly many transitions represented in a sparse fashion.
    /// Transitions are non-overlapping and ordered lexicographically by input
    /// range.
    ///
    /// In practice, this is used for encoding UTF-8 automata. Its presence is
    /// primarily an optimization that avoids many additional unconditional
    /// epsilon transitions (via [`Union`](State::Union) states), and thus
    /// decreases the overhead of traversing the NFA. This can improve both
    /// matching time and DFA construction time.
    Sparse(SparseTransitions),
    /// A dense representation of a state with multiple transitions.
    Dense(DenseTransitions),
    /// A conditional epsilon transition satisfied via some sort of
    /// look-around. Look-around is limited to anchor and word boundary
    /// assertions.
    ///
    /// Look-around states are meant to be evaluated while performing epsilon
    /// closure (computing the set of states reachable from a particular state
    /// via only epsilon transitions). If the current position in the haystack
    /// satisfies the look-around assertion, then you're permitted to follow
    /// that epsilon transition.
    Look {
        /// The look-around assertion that must be satisfied before moving
        /// to `next`.
        look: Look,
        /// The state to transition to if the look-around assertion is
        /// satisfied.
        next: StateID,
    },
    /// An alternation such that there exists an epsilon transition to all
    /// states in `alternates`, where matches found via earlier transitions
    /// are preferred over later transitions.
    Union {
        /// An ordered sequence of unconditional epsilon transitions to other
        /// states. Transitions earlier in the sequence are preferred over
        /// transitions later in the sequence.
        alternates: Box<[StateID]>,
    },
    /// An alternation such that there exists precisely two unconditional
    /// epsilon transitions, where matches found via `alt1` are preferred over
    /// matches found via `alt2`.
    ///
    /// This state exists as a common special case of Union where there are
    /// only two alternates. In this case, we don't need any allocations to
    /// represent the state. This saves a bit of memory and also saves an
    /// additional memory access when traversing the NFA.
    BinaryUnion {
        /// An unconditional epsilon transition to another NFA state. This
        /// is preferred over `alt2`.
        alt1: StateID,
        /// An unconditional epsilon transition to another NFA state. Matches
        /// reported via this transition should only be reported if no matches
        /// were found by following `alt1`.
        alt2: StateID,
    },
    /// An empty state that records a capture location.
    ///
    /// From the perspective of finite automata, this is precisely equivalent
    /// to an unconditional epsilon transition, but serves the purpose of
    /// instructing NFA simulations to record additional state when the finite
    /// state machine passes through this epsilon transition.
    ///
    /// `slot` in this context refers to the specific capture group slot
    /// offset that is being recorded. Each capturing group has two slots
    /// corresponding to the start and end of the matching portion of that
    /// group.
    ///
    /// The pattern ID and capture group index are also included in this state
    /// in case they are useful. But mostly, all you'll need is `next` and
    /// `slot`.
    Capture {
        /// The state to transition to, unconditionally.
        next: StateID,
        /// The pattern ID that this capture belongs to.
        pattern_id: PatternID,
        /// The capture group index that this capture belongs to. Capture group
        /// indices are local to each pattern. For example, when capturing
        /// groups are enabled, every pattern has a capture group at index
        /// `0`.
        group_index: SmallIndex,
        /// The slot index for this capture. Every capturing group has two
        /// slots: one for the start haystack offset and one for the end
        /// haystack offset. Unlike capture group indices, slot indices are
        /// global across all patterns in this NFA. That is, each slot belongs
        /// to a single pattern, but there is only one slot at index `i`.
        slot: SmallIndex,
    },
    /// A state that cannot be transitioned out of. This is useful for cases
    /// where you want to prevent matching from occurring. For example, if your
    /// regex parser permits empty character classes, then one could choose
    /// a `Fail` state to represent them. (An empty character class can be
    /// thought of as an empty set. Since nothing is in an empty set, they can
    /// never match anything.)
    Fail,
    /// A match state. There is at least one such occurrence of this state for
    /// each regex that can match that is in this NFA.
    Match {
        /// The matching pattern ID.
        pattern_id: PatternID,
    },
}
impl Start {
    pub(crate) fn from_usize(n: usize) -> Option<Start> {}
    pub(crate) fn len() -> usize {
        6
    }
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn as_u8(&self) -> u8 {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn as_usize(&self) -> usize {}
}
impl NFA {
    #[cfg(feature = "syntax")]
    pub fn new(pattern: &str) -> Result<NFA, BuildError> {}
    #[cfg(feature = "syntax")]
    pub fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<NFA, BuildError> {}
    pub fn always_match() -> NFA {}
    pub fn never_match() -> NFA {}
    #[cfg(feature = "syntax")]
    pub fn config() -> Config {}
    #[cfg(feature = "syntax")]
    pub fn compiler() -> Compiler {}
    pub fn patterns(&self) -> PatternIter<'_> {}
    #[inline]
    pub fn pattern_len(&self) -> usize {
        self.0.start_pattern.len()
    }
    #[inline]
    pub fn start_anchored(&self) -> StateID {}
    #[inline]
    pub fn start_unanchored(&self) -> StateID {}
    #[inline]
    pub fn start_pattern(&self, pid: PatternID) -> Option<StateID> {}
    #[inline]
    pub(crate) fn byte_class_set(&self) -> &ByteClassSet {}
    #[inline]
    pub fn byte_classes(&self) -> &ByteClasses {}
    #[inline]
    pub fn state(&self, id: StateID) -> &State {}
    #[inline]
    pub fn states(&self) -> &[State] {
        &self.0.states
    }
    #[inline]
    pub fn group_info(&self) -> &GroupInfo {}
    #[inline]
    pub fn has_capture(&self) -> bool {}
    #[inline]
    pub fn has_empty(&self) -> bool {}
    #[inline]
    pub fn is_utf8(&self) -> bool {}
    #[inline]
    pub fn is_reverse(&self) -> bool {}
    #[inline]
    pub fn is_always_start_anchored(&self) -> bool {}
    #[inline]
    pub fn look_matcher(&self) -> &LookMatcher {}
    #[inline]
    pub fn look_set_any(&self) -> LookSet {}
    #[inline]
    pub fn look_set_prefix_any(&self) -> LookSet {}
    #[inline]
    pub fn memory_usage(&self) -> usize {}
}
impl State {
    pub(crate) fn dead() -> State {
        StateBuilderEmpty::new().into_matches().into_nfa().to_state()
    }
    pub(crate) fn is_match(&self) -> bool {}
    pub(crate) fn is_from_word(&self) -> bool {}
    pub(crate) fn is_half_crlf(&self) -> bool {}
    pub(crate) fn look_have(&self) -> LookSet {}
    pub(crate) fn look_need(&self) -> LookSet {}
    pub(crate) fn match_len(&self) -> usize {}
    pub(crate) fn match_pattern(&self, index: usize) -> PatternID {}
    pub(crate) fn match_pattern_ids(&self) -> Option<Vec<PatternID>> {}
    #[cfg(all(test, not(miri)))]
    pub(crate) fn iter_match_pattern_ids<F: FnMut(PatternID)>(&self, f: F) {}
    pub(crate) fn iter_nfa_state_ids<F: FnMut(StateID)>(&self, f: F) {}
    pub(crate) fn memory_usage(&self) -> usize {
        self.0.len()
    }
    fn repr(&self) -> Repr<'_> {}
}
impl ByteClasses {
    #[inline]
    pub fn empty() -> ByteClasses {}
    #[inline]
    pub fn singletons() -> ByteClasses {}
    pub(crate) fn from_bytes(
        slice: &[u8],
    ) -> Result<(ByteClasses, usize), DeserializeError> {}
    pub(crate) fn write_to(&self, mut dst: &mut [u8]) -> Result<usize, SerializeError> {}
    pub(crate) fn write_to_len(&self) -> usize {}
    #[inline]
    pub fn set(&mut self, byte: u8, class: u8) {}
    #[inline]
    pub fn get(&self, byte: u8) -> u8 {}
    #[inline]
    pub fn get_by_unit(&self, unit: Unit) -> usize {}
    #[inline]
    pub fn eoi(&self) -> Unit {}
    #[inline]
    pub fn alphabet_len(&self) -> usize {}
    #[inline]
    pub fn stride2(&self) -> usize {
        let zeros = self.alphabet_len().next_power_of_two().trailing_zeros();
        usize::try_from(zeros).unwrap()
    }
    #[inline]
    pub fn is_singleton(&self) -> bool {}
    #[inline]
    pub fn iter(&self) -> ByteClassIter<'_> {}
    pub fn representatives<R: core::ops::RangeBounds<u8>>(
        &self,
        range: R,
    ) -> ByteClassRepresentatives<'_> {}
    #[inline]
    pub fn elements(&self, class: Unit) -> ByteClassElements {}
    fn element_ranges(&self, class: Unit) -> ByteClassElementRanges {}
}
fn minimum_cache_capacity(
    nfa: &thompson::NFA,
    classes: &ByteClasses,
    starts_for_each_pattern: bool,
) -> usize {
    const ID_SIZE: usize = size_of::<LazyStateID>();
    const STATE_SIZE: usize = size_of::<State>();
    let stride = 1 << classes.stride2();
    let states_len = nfa.states().len();
    let sparses = 2 * states_len * NFAStateID::SIZE;
    let trans = MIN_STATES * stride * ID_SIZE;
    let mut starts = Start::len() * ID_SIZE;
    if starts_for_each_pattern {
        starts += (Start::len() * nfa.pattern_len()) * ID_SIZE;
    }
    assert!(MIN_STATES >= 5, "minimum number of states has to be at least 5");
    let non_sentinel = MIN_STATES.checked_sub(SENTINEL_STATES).unwrap();
    let dead_state_size = State::dead().memory_usage();
    let max_state_size = 5 + 4 + (nfa.pattern_len() * 4) + (states_len * 5);
    let states = (SENTINEL_STATES * (STATE_SIZE + dead_state_size))
        + (non_sentinel * (STATE_SIZE + max_state_size));
    let states_to_sid = (MIN_STATES * STATE_SIZE) + (MIN_STATES * ID_SIZE);
    let stack = states_len * NFAStateID::SIZE;
    let scratch_state_builder = max_state_size;
    trans + starts + states + states_to_sid + sparses + stack + scratch_state_builder
}
