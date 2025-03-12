use alloc::vec::Vec;
pub(crate) use self::state::{
    State, StateBuilderEmpty, StateBuilderMatches, StateBuilderNFA,
};
use crate::{
    nfa::thompson,
    util::{
        alphabet, look::{Look, LookSet},
        primitives::StateID, search::MatchKind, sparse_set::{SparseSet, SparseSets},
        start::Start, utf8,
    },
};
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Transition {
    /// The inclusive start of the byte range.
    pub start: u8,
    /// The inclusive end of the byte range.
    pub end: u8,
    /// The identifier of the state to transition to.
    pub next: StateID,
}
#[derive(Clone, Debug)]
pub(crate) struct StateBuilderEmpty(Vec<u8>);
#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub(crate) struct State(Arc<[u8]>);
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct LookSet {
    /// The underlying representation this set is exposed to make it possible
    /// to store it somewhere efficiently. The representation is that
    /// of a bitset, where each assertion occupies bit `i` where
    /// `i = Look::as_repr()`.
    ///
    /// Note that users of this internal representation must permit the full
    /// range of `u16` values to be represented. For example, even if the
    /// current implementation only makes use of the 10 least significant bits,
    /// it may use more bits in a future semver compatible release.
    pub bits: u32,
}
#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct Unit(UnitKind);
#[derive(Clone)]
pub struct NFA(Arc<Inner>);
#[derive(Clone, Debug)]
pub struct LookMatcher {
    lineterm: DebugByte,
}
#[derive(Clone, Debug)]
pub(crate) struct SparseSets {
    pub(crate) set1: SparseSet,
    pub(crate) set2: SparseSet,
}
#[derive(Clone)]
pub(crate) struct StateBuilderMatches(Vec<u8>);
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SparseTransitions {
    /// The sorted sequence of non-overlapping transitions.
    pub transitions: Box<[Transition]>,
}
#[derive(Clone)]
pub(crate) struct SparseSet {
    /// The number of elements currently in this set.
    len: usize,
    /// Dense contains the ids in the order in which they were inserted.
    dense: Vec<StateID>,
    /// Sparse maps ids to their location in dense.
    ///
    /// A state ID is in the set if and only if
    /// sparse[id] < len && id == dense[sparse[id]].
    ///
    /// Note that these are indices into 'dense'. It's a little weird to use
    /// StateID here, but we know our length can never exceed the bounds of
    /// StateID (enforced by 'resize') and StateID will be at most 4 bytes
    /// where as a usize is likely double that in most cases.
    sparse: Vec<StateID>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DenseTransitions {
    /// A dense representation of this state's transitions on the heap. This
    /// always has length 256.
    pub transitions: Box<[StateID]>,
}
#[derive(Debug)]
pub(crate) struct SparseSetIter<'a>(core::slice::Iter<'a, StateID>);
#[derive(Clone)]
pub(crate) struct StateBuilderNFA {
    repr: Vec<u8>,
    prev_nfa_state_id: StateID,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MatchKind {
    /// Report all possible matches.
    All,
    /// Report only the leftmost matches. When multiple leftmost matches exist,
    /// report the match corresponding to the part of the regex that appears
    /// first in the syntax.
    LeftmostFirst,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Look {
    /// Match the beginning of text. Specifically, this matches at the starting
    /// position of the input.
    Start = 1 << 0,
    /// Match the end of text. Specifically, this matches at the ending
    /// position of the input.
    End = 1 << 1,
    /// Match the beginning of a line or the beginning of text. Specifically,
    /// this matches at the starting position of the input, or at the position
    /// immediately following a `\n` character.
    StartLF = 1 << 2,
    /// Match the end of a line or the end of text. Specifically, this matches
    /// at the end position of the input, or at the position immediately
    /// preceding a `\n` character.
    EndLF = 1 << 3,
    /// Match the beginning of a line or the beginning of text. Specifically,
    /// this matches at the starting position of the input, or at the position
    /// immediately following either a `\r` or `\n` character, but never after
    /// a `\r` when a `\n` follows.
    StartCRLF = 1 << 4,
    /// Match the end of a line or the end of text. Specifically, this matches
    /// at the end position of the input, or at the position immediately
    /// preceding a `\r` or `\n` character, but never before a `\n` when a `\r`
    /// precedes it.
    EndCRLF = 1 << 5,
    /// Match an ASCII-only word boundary. That is, this matches a position
    /// where the left adjacent character and right adjacent character
    /// correspond to a word and non-word or a non-word and word character.
    WordAscii = 1 << 6,
    /// Match an ASCII-only negation of a word boundary.
    WordAsciiNegate = 1 << 7,
    /// Match a Unicode-aware word boundary. That is, this matches a position
    /// where the left adjacent character and right adjacent character
    /// correspond to a word and non-word or a non-word and word character.
    WordUnicode = 1 << 8,
    /// Match a Unicode-aware negation of a word boundary.
    WordUnicodeNegate = 1 << 9,
    /// Match the start of an ASCII-only word boundary. That is, this matches a
    /// position at either the beginning of the haystack or where the previous
    /// character is not a word character and the following character is a word
    /// character.
    WordStartAscii = 1 << 10,
    /// Match the end of an ASCII-only word boundary. That is, this matches
    /// a position at either the end of the haystack or where the previous
    /// character is a word character and the following character is not a word
    /// character.
    WordEndAscii = 1 << 11,
    /// Match the start of a Unicode word boundary. That is, this matches a
    /// position at either the beginning of the haystack or where the previous
    /// character is not a word character and the following character is a word
    /// character.
    WordStartUnicode = 1 << 12,
    /// Match the end of a Unicode word boundary. That is, this matches a
    /// position at either the end of the haystack or where the previous
    /// character is a word character and the following character is not a word
    /// character.
    WordEndUnicode = 1 << 13,
    /// Match the start half of an ASCII-only word boundary. That is, this
    /// matches a position at either the beginning of the haystack or where the
    /// previous character is not a word character.
    WordStartHalfAscii = 1 << 14,
    /// Match the end half of an ASCII-only word boundary. That is, this
    /// matches a position at either the end of the haystack or where the
    /// following character is not a word character.
    WordEndHalfAscii = 1 << 15,
    /// Match the start half of a Unicode word boundary. That is, this matches
    /// a position at either the beginning of the haystack or where the
    /// previous character is not a word character.
    WordStartHalfUnicode = 1 << 16,
    /// Match the end half of a Unicode word boundary. That is, this matches
    /// a position at either the end of the haystack or where the following
    /// character is not a word character.
    WordEndHalfUnicode = 1 << 17,
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
impl Transition {
    pub fn matches(&self, haystack: &[u8], at: usize) -> bool {}
    pub fn matches_unit(&self, unit: alphabet::Unit) -> bool {
        unit.as_u8().map_or(false, |byte| self.matches_byte(byte))
    }
    pub fn matches_byte(&self, byte: u8) -> bool {}
}
impl StateBuilderEmpty {
    pub(crate) fn new() -> StateBuilderEmpty {}
    pub(crate) fn into_matches(mut self) -> StateBuilderMatches {
        self.0.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0, 0, 0]);
        StateBuilderMatches(self.0)
    }
    fn clear(&mut self) {}
    pub(crate) fn capacity(&self) -> usize {}
}
impl State {
    pub(crate) fn dead() -> State {}
    pub(crate) fn is_match(&self) -> bool {}
    pub(crate) fn is_from_word(&self) -> bool {
        self.repr().is_from_word()
    }
    pub(crate) fn is_half_crlf(&self) -> bool {
        self.repr().is_half_crlf()
    }
    pub(crate) fn look_have(&self) -> LookSet {
        self.repr().look_have()
    }
    pub(crate) fn look_need(&self) -> LookSet {
        self.repr().look_need()
    }
    pub(crate) fn match_len(&self) -> usize {}
    pub(crate) fn match_pattern(&self, index: usize) -> PatternID {}
    pub(crate) fn match_pattern_ids(&self) -> Option<Vec<PatternID>> {}
    #[cfg(all(test, not(miri)))]
    pub(crate) fn iter_match_pattern_ids<F: FnMut(PatternID)>(&self, f: F) {}
    pub(crate) fn iter_nfa_state_ids<F: FnMut(StateID)>(&self, f: F) {
        self.repr().iter_nfa_state_ids(f)
    }
    pub(crate) fn memory_usage(&self) -> usize {}
    fn repr(&self) -> Repr<'_> {}
}
impl LookSet {
    #[inline]
    pub fn empty() -> LookSet {}
    #[inline]
    pub fn full() -> LookSet {}
    #[inline]
    pub fn singleton(look: Look) -> LookSet {}
    #[inline]
    pub fn len(self) -> usize {}
    #[inline]
    pub fn is_empty(self) -> bool {
        self.len() == 0
    }
    #[inline]
    pub fn contains(self, look: Look) -> bool {}
    #[inline]
    pub fn contains_anchor(&self) -> bool {}
    #[inline]
    pub fn contains_anchor_haystack(&self) -> bool {}
    #[inline]
    pub fn contains_anchor_line(&self) -> bool {
        self.contains(Look::StartLF) || self.contains(Look::EndLF)
            || self.contains(Look::StartCRLF) || self.contains(Look::EndCRLF)
    }
    #[inline]
    pub fn contains_anchor_lf(&self) -> bool {}
    #[inline]
    pub fn contains_anchor_crlf(&self) -> bool {
        self.contains(Look::StartCRLF) || self.contains(Look::EndCRLF)
    }
    #[inline]
    pub fn contains_word(self) -> bool {
        self.contains_word_unicode() || self.contains_word_ascii()
    }
    #[inline]
    pub fn contains_word_unicode(self) -> bool {}
    #[inline]
    pub fn contains_word_ascii(self) -> bool {}
    #[inline]
    pub fn iter(self) -> LookSetIter {}
    #[inline]
    pub fn insert(self, look: Look) -> LookSet {
        LookSet {
            bits: self.bits | look.as_repr(),
        }
    }
    #[inline]
    pub fn set_insert(&mut self, look: Look) {}
    #[inline]
    pub fn remove(self, look: Look) -> LookSet {}
    #[inline]
    pub fn set_remove(&mut self, look: Look) {}
    #[inline]
    pub fn subtract(self, other: LookSet) -> LookSet {
        LookSet {
            bits: self.bits & !other.bits,
        }
    }
    #[inline]
    pub fn set_subtract(&mut self, other: LookSet) {}
    #[inline]
    pub fn union(self, other: LookSet) -> LookSet {}
    #[inline]
    pub fn set_union(&mut self, other: LookSet) {}
    #[inline]
    pub fn intersect(self, other: LookSet) -> LookSet {
        LookSet {
            bits: self.bits & other.bits,
        }
    }
    #[inline]
    pub fn set_intersect(&mut self, other: LookSet) {}
    #[inline]
    pub fn read_repr(slice: &[u8]) -> LookSet {}
    #[inline]
    pub fn write_repr(self, slice: &mut [u8]) {}
    pub fn available(self) -> Result<(), UnicodeWordBoundaryError> {}
}
impl Unit {
    pub fn u8(byte: u8) -> Unit {}
    pub fn eoi(num_byte_equiv_classes: usize) -> Unit {}
    pub fn as_u8(self) -> Option<u8> {
        match self.0 {
            UnitKind::U8(b) => Some(b),
            UnitKind::EOI(_) => None,
        }
    }
    pub fn as_eoi(self) -> Option<u16> {}
    pub fn as_usize(self) -> usize {}
    pub fn is_byte(self, byte: u8) -> bool {
        self.as_u8().map_or(false, |b| b == byte)
    }
    pub fn is_eoi(self) -> bool {}
    pub fn is_word_byte(self) -> bool {
        self.as_u8().map_or(false, crate::util::utf8::is_word_byte)
    }
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
    pub fn pattern_len(&self) -> usize {}
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
    pub fn state(&self, id: StateID) -> &State {
        &self.states()[id]
    }
    #[inline]
    pub fn states(&self) -> &[State] {}
    #[inline]
    pub fn group_info(&self) -> &GroupInfo {}
    #[inline]
    pub fn has_capture(&self) -> bool {}
    #[inline]
    pub fn has_empty(&self) -> bool {}
    #[inline]
    pub fn is_utf8(&self) -> bool {}
    #[inline]
    pub fn is_reverse(&self) -> bool {
        self.0.reverse
    }
    #[inline]
    pub fn is_always_start_anchored(&self) -> bool {}
    #[inline]
    pub fn look_matcher(&self) -> &LookMatcher {
        &self.0.look_matcher
    }
    #[inline]
    pub fn look_set_any(&self) -> LookSet {
        self.0.look_set_any
    }
    #[inline]
    pub fn look_set_prefix_any(&self) -> LookSet {}
    #[inline]
    pub fn memory_usage(&self) -> usize {}
}
impl LookMatcher {
    pub fn new() -> LookMatcher {}
    pub fn set_line_terminator(&mut self, byte: u8) -> &mut LookMatcher {}
    pub fn get_line_terminator(&self) -> u8 {
        self.lineterm.0
    }
    #[inline]
    pub fn matches(&self, look: Look, haystack: &[u8], at: usize) -> bool {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn matches_inline(&self, look: Look, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn matches_set(&self, set: LookSet, haystack: &[u8], at: usize) -> bool {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn matches_set_inline(
        &self,
        set: LookSet,
        haystack: &[u8],
        at: usize,
    ) -> bool {}
    #[cfg(feature = "alloc")]
    pub(crate) fn add_to_byteset(
        &self,
        look: Look,
        set: &mut crate::util::alphabet::ByteClassSet,
    ) {}
    #[inline]
    pub fn is_start(&self, _haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_end(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_start_lf(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_end_lf(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_start_crlf(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_end_crlf(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_word_ascii(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_word_ascii_negate(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_word_unicode(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, UnicodeWordBoundaryError> {}
    #[inline]
    pub fn is_word_unicode_negate(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, UnicodeWordBoundaryError> {}
    #[inline]
    pub fn is_word_start_ascii(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_word_end_ascii(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_word_start_unicode(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, UnicodeWordBoundaryError> {}
    #[inline]
    pub fn is_word_end_unicode(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, UnicodeWordBoundaryError> {}
    #[inline]
    pub fn is_word_start_half_ascii(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_word_end_half_ascii(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_word_start_half_unicode(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, UnicodeWordBoundaryError> {}
    #[inline]
    pub fn is_word_end_half_unicode(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, UnicodeWordBoundaryError> {}
}
impl SparseSets {
    pub(crate) fn new(capacity: usize) -> SparseSets {}
    #[inline]
    pub(crate) fn resize(&mut self, new_capacity: usize) {}
    pub(crate) fn clear(&mut self) {
        self.set1.clear();
        self.set2.clear();
    }
    pub(crate) fn swap(&mut self) {
        core::mem::swap(&mut self.set1, &mut self.set2);
    }
    pub(crate) fn memory_usage(&self) -> usize {}
}
impl StateBuilderMatches {
    pub(crate) fn into_nfa(mut self) -> StateBuilderNFA {
        self.repr_vec().close_match_pattern_ids();
        StateBuilderNFA {
            repr: self.0,
            prev_nfa_state_id: StateID::ZERO,
        }
    }
    pub(crate) fn set_is_from_word(&mut self) {
        self.repr_vec().set_is_from_word()
    }
    pub(crate) fn set_is_half_crlf(&mut self) {
        self.repr_vec().set_is_half_crlf()
    }
    pub(crate) fn look_have(&self) -> LookSet {
        LookSet::read_repr(&self.0[1..])
    }
    pub(crate) fn set_look_have(&mut self, set: impl FnMut(LookSet) -> LookSet) {
        self.repr_vec().set_look_have(set)
    }
    pub(crate) fn add_match_pattern_id(&mut self, pid: PatternID) {
        self.repr_vec().add_match_pattern_id(pid)
    }
    fn repr(&self) -> Repr<'_> {}
    fn repr_vec(&mut self) -> ReprVec<'_> {}
}
impl SparseTransitions {
    #[inline]
    pub fn matches(&self, haystack: &[u8], at: usize) -> Option<StateID> {}
    #[inline]
    pub(crate) fn matches_unit(&self, unit: alphabet::Unit) -> Option<StateID> {
        unit.as_u8().map_or(None, |byte| self.matches_byte(byte))
    }
    #[inline]
    pub fn matches_byte(&self, byte: u8) -> Option<StateID> {}
}
impl SparseSet {
    #[inline]
    pub(crate) fn new(capacity: usize) -> SparseSet {}
    #[inline]
    pub(crate) fn resize(&mut self, new_capacity: usize) {}
    #[inline]
    pub(crate) fn capacity(&self) -> usize {}
    #[inline]
    pub(crate) fn len(&self) -> usize {}
    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        self.len() == 0
    }
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn insert(&mut self, id: StateID) -> bool {}
    #[inline]
    pub(crate) fn contains(&self, id: StateID) -> bool {}
    #[inline]
    pub(crate) fn clear(&mut self) {
        self.len = 0;
    }
    #[inline]
    pub(crate) fn iter(&self) -> SparseSetIter<'_> {
        SparseSetIter(self.dense[..self.len()].iter())
    }
    #[inline]
    pub(crate) fn memory_usage(&self) -> usize {}
}
impl DenseTransitions {
    #[inline]
    pub fn matches(&self, haystack: &[u8], at: usize) -> Option<StateID> {}
    #[inline]
    pub(crate) fn matches_unit(&self, unit: alphabet::Unit) -> Option<StateID> {
        unit.as_u8().map_or(None, |byte| self.matches_byte(byte))
    }
    #[inline]
    pub fn matches_byte(&self, byte: u8) -> Option<StateID> {}
    pub(crate) fn iter(&self) -> impl Iterator<Item = Transition> + '_ {}
}
impl MatchKind {
    #[cfg(feature = "alloc")]
    pub(crate) fn continue_past_first_match(&self) -> bool {
        *self == MatchKind::All
    }
}
pub(crate) fn next(
    nfa: &thompson::NFA,
    match_kind: MatchKind,
    sparses: &mut SparseSets,
    stack: &mut Vec<StateID>,
    state: &State,
    unit: alphabet::Unit,
    empty_builder: StateBuilderEmpty,
) -> StateBuilderNFA {
    sparses.clear();
    let rev = nfa.is_reverse();
    let lookm = nfa.look_matcher();
    state
        .iter_nfa_state_ids(|nfa_id| {
            sparses.set1.insert(nfa_id);
        });
    if !state.look_need().is_empty() {
        let mut look_have = state.look_have().clone();
        match unit.as_u8() {
            Some(b'\r') => {
                if !rev || !state.is_half_crlf() {
                    look_have = look_have.insert(Look::EndCRLF);
                }
            }
            Some(b'\n') => {
                if rev || !state.is_half_crlf() {
                    look_have = look_have.insert(Look::EndCRLF);
                }
            }
            Some(_) => {}
            None => {
                look_have = look_have
                    .insert(Look::End)
                    .insert(Look::EndLF)
                    .insert(Look::EndCRLF);
            }
        }
        if unit.is_byte(lookm.get_line_terminator()) {
            look_have = look_have.insert(Look::EndLF);
        }
        if state.is_half_crlf()
            && ((rev && !unit.is_byte(b'\r')) || (!rev && !unit.is_byte(b'\n')))
        {
            look_have = look_have.insert(Look::StartCRLF);
        }
        if state.is_from_word() == unit.is_word_byte() {
            look_have = look_have
                .insert(Look::WordAsciiNegate)
                .insert(Look::WordUnicodeNegate);
        } else {
            look_have = look_have.insert(Look::WordAscii).insert(Look::WordUnicode);
        }
        if !unit.is_word_byte() {
            look_have = look_have
                .insert(Look::WordEndHalfAscii)
                .insert(Look::WordEndHalfUnicode);
        }
        if state.is_from_word() && !unit.is_word_byte() {
            look_have = look_have
                .insert(Look::WordEndAscii)
                .insert(Look::WordEndUnicode);
        } else if !state.is_from_word() && unit.is_word_byte() {
            look_have = look_have
                .insert(Look::WordStartAscii)
                .insert(Look::WordStartUnicode);
        }
        if !look_have.subtract(state.look_have()).intersect(state.look_need()).is_empty()
        {
            for nfa_id in sparses.set1.iter() {
                epsilon_closure(nfa, nfa_id, look_have, stack, &mut sparses.set2);
            }
            sparses.swap();
            sparses.set2.clear();
        }
    }
    let mut builder = empty_builder.into_matches();
    if nfa.look_set_any().contains_anchor_line()
        && unit.is_byte(lookm.get_line_terminator())
    {
        builder.set_look_have(|have| have.insert(Look::StartLF));
    }
    if nfa.look_set_any().contains_anchor_crlf()
        && ((rev && unit.is_byte(b'\r')) || (!rev && unit.is_byte(b'\n')))
    {
        builder.set_look_have(|have| have.insert(Look::StartCRLF));
    }
    if nfa.look_set_any().contains_word() && !unit.is_word_byte() {
        builder
            .set_look_have(|have| {
                have.insert(Look::WordStartHalfAscii).insert(Look::WordStartHalfUnicode)
            });
    }
    for nfa_id in sparses.set1.iter() {
        match *nfa.state(nfa_id) {
            thompson::State::Union { .. }
            | thompson::State::BinaryUnion { .. }
            | thompson::State::Fail
            | thompson::State::Look { .. }
            | thompson::State::Capture { .. } => {}
            thompson::State::Match { pattern_id } => {
                builder.add_match_pattern_id(pattern_id);
                if !match_kind.continue_past_first_match() {
                    break;
                }
            }
            thompson::State::ByteRange { ref trans } => {
                if trans.matches_unit(unit) {
                    epsilon_closure(
                        nfa,
                        trans.next,
                        builder.look_have(),
                        stack,
                        &mut sparses.set2,
                    );
                }
            }
            thompson::State::Sparse(ref sparse) => {
                if let Some(next) = sparse.matches_unit(unit) {
                    epsilon_closure(
                        nfa,
                        next,
                        builder.look_have(),
                        stack,
                        &mut sparses.set2,
                    );
                }
            }
            thompson::State::Dense(ref dense) => {
                if let Some(next) = dense.matches_unit(unit) {
                    epsilon_closure(
                        nfa,
                        next,
                        builder.look_have(),
                        stack,
                        &mut sparses.set2,
                    );
                }
            }
        }
    }
    if !sparses.set2.is_empty() {
        if nfa.look_set_any().contains_word() && unit.is_word_byte() {
            builder.set_is_from_word();
        }
        if nfa.look_set_any().contains_anchor_crlf()
            && ((rev && unit.is_byte(b'\n')) || (!rev && unit.is_byte(b'\r')))
        {
            builder.set_is_half_crlf();
        }
    }
    let mut builder_nfa = builder.into_nfa();
    add_nfa_states(nfa, &sparses.set2, &mut builder_nfa);
    builder_nfa
}
pub(crate) fn epsilon_closure(
    nfa: &thompson::NFA,
    start_nfa_id: StateID,
    look_have: LookSet,
    stack: &mut Vec<StateID>,
    set: &mut SparseSet,
) {
    assert!(stack.is_empty());
    if !nfa.state(start_nfa_id).is_epsilon() {
        set.insert(start_nfa_id);
        return;
    }
    stack.push(start_nfa_id);
    while let Some(mut id) = stack.pop() {
        loop {
            if !set.insert(id) {
                break;
            }
            match *nfa.state(id) {
                thompson::State::ByteRange { .. }
                | thompson::State::Sparse { .. }
                | thompson::State::Dense { .. }
                | thompson::State::Fail
                | thompson::State::Match { .. } => break,
                thompson::State::Look { look, next } => {
                    if !look_have.contains(look) {
                        break;
                    }
                    id = next;
                }
                thompson::State::Union { ref alternates } => {
                    id = match alternates.get(0) {
                        None => break,
                        Some(&id) => id,
                    };
                    stack.extend(alternates[1..].iter().rev());
                }
                thompson::State::BinaryUnion { alt1, alt2 } => {
                    id = alt1;
                    stack.push(alt2);
                }
                thompson::State::Capture { next, .. } => {
                    id = next;
                }
            }
        }
    }
}
pub(crate) fn add_nfa_states(
    nfa: &thompson::NFA,
    set: &SparseSet,
    builder: &mut StateBuilderNFA,
) {
    for nfa_id in set.iter() {
        match *nfa.state(nfa_id) {
            thompson::State::ByteRange { .. } => {
                builder.add_nfa_state_id(nfa_id);
            }
            thompson::State::Sparse { .. } => {
                builder.add_nfa_state_id(nfa_id);
            }
            thompson::State::Dense { .. } => {
                builder.add_nfa_state_id(nfa_id);
            }
            thompson::State::Look { look, .. } => {
                builder.add_nfa_state_id(nfa_id);
                builder.set_look_need(|need| need.insert(look));
            }
            thompson::State::Union { .. } | thompson::State::BinaryUnion { .. } => {
                builder.add_nfa_state_id(nfa_id);
            }
            thompson::State::Capture { .. } => {}
            thompson::State::Fail => {
                builder.add_nfa_state_id(nfa_id);
            }
            thompson::State::Match { .. } => {
                builder.add_nfa_state_id(nfa_id);
            }
        }
    }
    if builder.look_need().is_empty() {
        builder.set_look_have(|_| LookSet::empty());
    }
}
