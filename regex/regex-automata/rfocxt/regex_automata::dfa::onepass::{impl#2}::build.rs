use alloc::{vec, vec::Vec};
use crate::{
    dfa::{remapper::Remapper, DEAD},
    nfa::thompson::{self, NFA},
    util::{
        alphabet::ByteClasses, captures::Captures, escape::DebugByte,
        int::{Usize, U32, U64, U8},
        look::{Look, LookSet, UnicodeWordBoundaryError},
        primitives::{NonMaxUsize, PatternID, StateID},
        search::{Anchored, Input, Match, MatchError, MatchKind, Span},
        sparse_set::SparseSet,
    },
};
pub(crate) trait Usize {
    fn as_u8(self) -> u8;
    fn as_u16(self) -> u16;
    fn as_u32(self) -> u32;
    fn as_u64(self) -> u64;
}
#[derive(Debug)]
struct InternalBuilder<'a> {
    /// The DFA we're building.
    dfa: DFA,
    /// An unordered collection of NFA state IDs that we haven't yet tried to
    /// build into a DFA state yet.
    ///
    /// This collection does not ultimately wind up including every NFA state
    /// ID. Instead, each ID represents a "start" state for a sub-graph of the
    /// NFA. The set of NFA states we then use to build a DFA state consists
    /// of that "start" state and all states reachable from it via epsilon
    /// transitions.
    uncompiled_nfa_ids: Vec<StateID>,
    /// A map from NFA state ID to DFA state ID. This is useful for easily
    /// determining whether an NFA state has been used as a "starting" point
    /// to build a DFA state yet. If it hasn't, then it is mapped to DEAD,
    /// and since DEAD is specially added and never corresponds to any NFA
    /// state, it follows that a mapping to DEAD implies the NFA state has
    /// no corresponding DFA state yet.
    nfa_to_dfa_id: Vec<StateID>,
    /// A stack used to traverse the NFA states that make up a single DFA
    /// state. Traversal occurs until the stack is empty, and we only push to
    /// the stack when the state ID isn't in 'seen'. Actually, even more than
    /// that, if we try to push something on to this stack that is already in
    /// 'seen', then we bail out on construction completely, since it implies
    /// that the NFA is not one-pass.
    stack: Vec<(StateID, Epsilons)>,
    /// The set of NFA states that we've visited via 'stack'.
    seen: SparseSet,
    /// Whether a match NFA state has been observed while constructing a
    /// one-pass DFA state. Once a match state is seen, assuming we are using
    /// leftmost-first match semantics, then we don't add any more transitions
    /// to the DFA state we're building.
    matched: bool,
    /// The config passed to the builder.
    ///
    /// This is duplicated in dfa.config.
    config: Config,
    /// The NFA we're building a one-pass DFA from.
    ///
    /// This is duplicated in dfa.nfa.
    nfa: &'a NFA,
    /// The equivalence classes that make up the alphabet for this DFA>
    ///
    /// This is duplicated in dfa.classes.
    classes: ByteClasses,
}
#[derive(Clone, Debug)]
pub struct LookSetIter {
    set: LookSet,
}
#[derive(Clone, Debug)]
pub struct BuildError {
    kind: BuildErrorKind,
}
#[derive(Clone, Copy)]
struct PatternEpsilons(u64);
#[derive(Clone, Debug, Default)]
pub struct Config {
    match_kind: Option<MatchKind>,
    starts_for_each_pattern: Option<bool>,
    byte_classes: Option<bool>,
    size_limit: Option<Option<usize>>,
}
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct SmallIndex(u32);
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DenseTransitions {
    /// A dense representation of this state's transitions on the heap. This
    /// always has length 256.
    pub transitions: Box<[StateID]>,
}
#[derive(Clone)]
pub struct NFA(Arc<Inner>);
#[derive(Clone, Copy)]
struct Epsilons(u64);
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
#[derive(Clone)]
pub struct DFA {
    /// The configuration provided by the caller.
    config: Config,
    /// The NFA used to build this DFA.
    ///
    /// NOTE: We probably don't need to store the NFA here, but we use enough
    /// bits from it that it's convenient to do so. And there really isn't much
    /// cost to doing so either, since an NFA is reference counted internally.
    nfa: NFA,
    /// The transition table. Given a state ID 's' and a byte of haystack 'b',
    /// the next state is `table[sid + classes[byte]]`.
    ///
    /// The stride of this table (i.e., the number of columns) is always
    /// a power of 2, even if the alphabet length is smaller. This makes
    /// converting between state IDs and state indices very cheap.
    ///
    /// Note that the stride always includes room for one extra "transition"
    /// that isn't actually a transition. It is a 'PatternEpsilons' that is
    /// used for match states only. Because of this, the maximum number of
    /// active columns in the transition table is 257, which means the maximum
    /// stride is 512 (the next power of 2 greater than or equal to 257).
    table: Vec<Transition>,
    /// The DFA state IDs of the starting states.
    ///
    /// `starts[0]` is always present and corresponds to the starting state
    /// when searching for matches of any pattern in the DFA.
    ///
    /// `starts[i]` where i>0 corresponds to the starting state for the pattern
    /// ID 'i-1'. These starting states are optional.
    starts: Vec<StateID>,
    /// Every state ID >= this value corresponds to a match state.
    ///
    /// This is what a search uses to detect whether a state is a match state
    /// or not. It requires only a simple comparison instead of bit-unpacking
    /// the PatternEpsilons from every state.
    min_match_id: StateID,
    /// The alphabet of this DFA, split into equivalence classes. Bytes in the
    /// same equivalence class can never discriminate between a match and a
    /// non-match.
    classes: ByteClasses,
    /// The number of elements in each state in the transition table. This may
    /// be less than the stride, since the stride is always a power of 2 and
    /// the alphabet length can be anything up to and including 256.
    alphabet_len: usize,
    /// The number of columns in the transition table, expressed as a power of
    /// 2.
    stride2: usize,
    /// The offset at which the PatternEpsilons for a match state is stored in
    /// the transition table.
    ///
    /// PERF: One wonders whether it would be better to put this in a separate
    /// allocation, since only match states have a non-empty PatternEpsilons
    /// and the number of match states tends be dwarfed by the number of
    /// non-match states. So this would save '8*len(non_match_states)' for each
    /// DFA. The question is whether moving this to a different allocation will
    /// lead to a perf hit during searches. You might think dealing with match
    /// states is rare, but some regexes spend a lot of time in match states
    /// gobbling up input. But... match state handling is already somewhat
    /// expensive, so maybe this wouldn't do much? Either way, it's worth
    /// experimenting.
    pateps_offset: usize,
    /// The first explicit slot index. This refers to the first slot appearing
    /// immediately after the last implicit slot. It is always 'patterns.len()
    /// * 2'.
    ///
    /// We record this because we only store the explicit slots in our DFA
    /// transition table that need to be saved. Implicit slots are handled
    /// automatically as part of the search.
    explicit_slot_start: usize,
}
#[derive(Clone, Debug, Default)]
pub struct GroupInfo(Arc<GroupInfoInner>);
#[derive(Clone, Copy)]
struct Slots(u32);
#[derive(Clone, Debug)]
pub struct Config {
    look_behind: Option<u8>,
    anchored: Anchored,
}
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Transition {
    /// The inclusive start of the byte range.
    pub start: u8,
    /// The inclusive end of the byte range.
    pub end: u8,
    /// The identifier of the state to transition to.
    pub next: StateID,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
#[derive(Clone, Copy)]
pub struct ByteClasses([u8; 256]);
#[derive(Clone, Debug)]
pub(crate) struct Config {
    match_kind: MatchKind,
    quit: ByteSet,
    dfa_size_limit: Option<usize>,
    determinize_size_limit: Option<usize>,
}
#[derive(Clone, Debug, Default)]
pub struct Config {
    match_kind: Option<MatchKind>,
    pre: Option<Option<Prefilter>>,
    starts_for_each_pattern: Option<bool>,
    byte_classes: Option<bool>,
    unicode_word_boundary: Option<bool>,
    quitset: Option<ByteSet>,
    specialize_start_states: Option<bool>,
    cache_capacity: Option<usize>,
    skip_cache_capacity_check: Option<bool>,
    minimum_cache_clear_count: Option<Option<usize>>,
    minimum_bytes_per_state: Option<Option<usize>>,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
#[derive(Clone, Debug)]
pub struct DFA {
    config: Config,
    nfa: thompson::NFA,
    stride2: usize,
    start_map: StartByteMap,
    classes: ByteClasses,
    quitset: ByteSet,
    cache_capacity: usize,
}
#[derive(Clone, Debug, Default)]
pub struct Config {
    match_kind: Option<MatchKind>,
    pre: Option<Option<Prefilter>>,
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
#[derive(Debug)]
pub struct PatternIter<'a> {
    it: PatternIDIter,
    /// We explicitly associate a lifetime with this iterator even though we
    /// don't actually borrow anything from the NFA. We do this for backward
    /// compatibility purposes. If we ever do need to borrow something from
    /// the NFA, then we can and just get rid of this marker without breaking
    /// the public API.
    _marker: core::marker::PhantomData<&'a ()>,
}
#[derive(Clone, Debug, Default)]
pub struct Config {
    utf8: Option<bool>,
    reverse: Option<bool>,
    nfa_size_limit: Option<Option<usize>>,
    shrink: Option<bool>,
    which_captures: Option<WhichCaptures>,
    look_matcher: Option<LookMatcher>,
    #[cfg(test)]
    unanchored_prefix: Option<bool>,
}
#[derive(Debug)]
pub(crate) struct DFA(Option<DFAEngine>);
#[derive(Clone, Debug, Default)]
pub struct Config {
    match_kind: Option<MatchKind>,
    utf8_empty: Option<bool>,
    autopre: Option<bool>,
    pre: Option<Option<Prefilter>>,
    which_captures: Option<WhichCaptures>,
    nfa_size_limit: Option<Option<usize>>,
    onepass_size_limit: Option<Option<usize>>,
    hybrid_cache_capacity: Option<usize>,
    hybrid: Option<bool>,
    dfa: Option<bool>,
    dfa_size_limit: Option<Option<usize>>,
    dfa_state_limit: Option<Option<usize>>,
    onepass: Option<bool>,
    backtrack: Option<bool>,
    byte_classes: Option<bool>,
    line_terminator: Option<u8>,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SparseTransitions {
    /// The sorted sequence of non-overlapping transitions.
    pub transitions: Box<[Transition]>,
}
#[derive(Clone, Debug)]
pub struct UnicodeWordBoundaryError(());
#[derive(Clone)]
pub struct DFA<T> {
    tt: Transitions<T>,
    st: StartTable<T>,
    special: Special,
    pre: Option<Prefilter>,
    quitset: ByteSet,
    flags: Flags,
}
#[cfg(feature = "dfa-build")]
#[derive(Clone, Debug, Default)]
pub struct Config {
    accelerate: Option<bool>,
    pre: Option<Option<Prefilter>>,
    minimize: Option<bool>,
    match_kind: Option<MatchKind>,
    start_kind: Option<StartKind>,
    starts_for_each_pattern: Option<bool>,
    byte_classes: Option<bool>,
    unicode_word_boundary: Option<bool>,
    quitset: Option<ByteSet>,
    specialize_start_states: Option<bool>,
    dfa_size_limit: Option<Option<usize>>,
    determinize_size_limit: Option<Option<usize>>,
}
#[derive(Clone, Debug, Default)]
pub struct Config {
    pre: Option<Option<Prefilter>>,
    visited_capacity: Option<usize>,
}
#[derive(Clone, Copy, Debug)]
pub struct Config {
    case_insensitive: bool,
    multi_line: bool,
    dot_matches_new_line: bool,
    crlf: bool,
    line_terminator: u8,
    swap_greed: bool,
    ignore_whitespace: bool,
    unicode: bool,
    utf8: bool,
    nest_limit: u32,
    octal: bool,
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
impl<'a> InternalBuilder<'a> {
    fn new(config: Config, nfa: &'a NFA) -> InternalBuilder<'a> {}
    fn build(mut self) -> Result<DFA, BuildError> {
        self.nfa.look_set_any().available().map_err(BuildError::word)?;
        for look in self.nfa.look_set_any().iter() {
            if look.as_repr() > Look::WordUnicodeNegate.as_repr() {
                return Err(BuildError::unsupported_look(look));
            }
        }
        if self.nfa.pattern_len().as_u64() > PatternEpsilons::PATTERN_ID_LIMIT {
            return Err(BuildError::too_many_patterns(PatternEpsilons::PATTERN_ID_LIMIT));
        }
        if self.nfa.group_info().explicit_slot_len() > Slots::LIMIT {
            return Err(
                BuildError::not_one_pass(
                    "too many explicit capturing groups (max is 16)",
                ),
            );
        }
        assert_eq!(DEAD, self.add_empty_state() ?);
        let explicit_slot_start = self.nfa.pattern_len() * 2;
        self.add_start_state(None, self.nfa.start_anchored())?;
        if self.config.get_starts_for_each_pattern() {
            for pid in self.nfa.patterns() {
                self.add_start_state(Some(pid), self.nfa.start_pattern(pid).unwrap())?;
            }
        }
        while let Some(nfa_id) = self.uncompiled_nfa_ids.pop() {
            let dfa_id = self.nfa_to_dfa_id[nfa_id];
            self.matched = false;
            self.seen.clear();
            self.stack_push(nfa_id, Epsilons::empty())?;
            while let Some((id, epsilons)) = self.stack.pop() {
                match *self.nfa.state(id) {
                    thompson::State::ByteRange { ref trans } => {
                        self.compile_transition(dfa_id, trans, epsilons)?;
                    }
                    thompson::State::Sparse(ref sparse) => {
                        for trans in sparse.transitions.iter() {
                            self.compile_transition(dfa_id, trans, epsilons)?;
                        }
                    }
                    thompson::State::Dense(ref dense) => {
                        for trans in dense.iter() {
                            self.compile_transition(dfa_id, &trans, epsilons)?;
                        }
                    }
                    thompson::State::Look { look, next } => {
                        let looks = epsilons.looks().insert(look);
                        self.stack_push(next, epsilons.set_looks(looks))?;
                    }
                    thompson::State::Union { ref alternates } => {
                        for &sid in alternates.iter().rev() {
                            self.stack_push(sid, epsilons)?;
                        }
                    }
                    thompson::State::BinaryUnion { alt1, alt2 } => {
                        self.stack_push(alt2, epsilons)?;
                        self.stack_push(alt1, epsilons)?;
                    }
                    thompson::State::Capture { next, slot, .. } => {
                        let slot = slot.as_usize();
                        let epsilons = if slot < explicit_slot_start {
                            epsilons
                        } else {
                            let offset = slot - explicit_slot_start;
                            epsilons.set_slots(epsilons.slots().insert(offset))
                        };
                        self.stack_push(next, epsilons)?;
                    }
                    thompson::State::Fail => {
                        continue;
                    }
                    thompson::State::Match { pattern_id } => {
                        if self.matched {
                            return Err(
                                BuildError::not_one_pass(
                                    "multiple epsilon transitions to match state",
                                ),
                            );
                        }
                        self.matched = true;
                        self.dfa
                            .set_pattern_epsilons(
                                dfa_id,
                                PatternEpsilons::empty()
                                    .set_pattern_id(pattern_id)
                                    .set_epsilons(epsilons),
                            );
                    }
                }
            }
        }
        self.shuffle_states();
        Ok(self.dfa)
    }
    fn shuffle_states(&mut self) {
        let mut remapper = Remapper::new(&self.dfa);
        let mut next_dest = self.dfa.last_state_id();
        for i in (0..self.dfa.state_len()).rev() {
            let id = StateID::must(i);
            let is_match = self.dfa.pattern_epsilons(id).pattern_id().is_some();
            if !is_match {
                continue;
            }
            remapper.swap(&mut self.dfa, next_dest, id);
            self.dfa.min_match_id = next_dest;
            next_dest = self
                .dfa
                .prev_state_id(next_dest)
                .expect("match states should be a proper subset of all states");
        }
        remapper.remap(&mut self.dfa);
    }
    fn compile_transition(
        &mut self,
        dfa_id: StateID,
        trans: &thompson::Transition,
        epsilons: Epsilons,
    ) -> Result<(), BuildError> {
        let next_dfa_id = self.add_dfa_state_for_nfa_state(trans.next)?;
        for byte in self
            .classes
            .representatives(trans.start..=trans.end)
            .filter_map(|r| r.as_u8())
        {
            let oldtrans = self.dfa.transition(dfa_id, byte);
            let newtrans = Transition::new(self.matched, next_dfa_id, epsilons);
            if oldtrans.state_id() == DEAD {
                self.dfa.set_transition(dfa_id, byte, newtrans);
            } else if oldtrans != newtrans {
                return Err(BuildError::not_one_pass("conflicting transition"));
            }
        }
        Ok(())
    }
    fn add_start_state(
        &mut self,
        pid: Option<PatternID>,
        nfa_id: StateID,
    ) -> Result<StateID, BuildError> {
        match pid {
            None => assert!(self.dfa.starts.is_empty()),
            Some(pid) => assert!(self.dfa.starts.len() == pid.one_more()),
        }
        let dfa_id = self.add_dfa_state_for_nfa_state(nfa_id)?;
        self.dfa.starts.push(dfa_id);
        Ok(dfa_id)
    }
    fn add_dfa_state_for_nfa_state(
        &mut self,
        nfa_id: StateID,
    ) -> Result<StateID, BuildError> {}
    fn add_empty_state(&mut self) -> Result<StateID, BuildError> {
        let state_limit = Transition::STATE_ID_LIMIT;
        let next_id = self.dfa.table.len() >> self.dfa.stride2();
        let id = StateID::new(next_id)
            .map_err(|_| BuildError::too_many_states(state_limit))?;
        if id.as_u64() > Transition::STATE_ID_LIMIT {
            return Err(BuildError::too_many_states(state_limit));
        }
        self.dfa.table.extend(core::iter::repeat(Transition(0)).take(self.dfa.stride()));
        self.dfa.set_pattern_epsilons(id, PatternEpsilons::empty());
        if let Some(size_limit) = self.config.get_size_limit() {
            if self.dfa.memory_usage() > size_limit {
                return Err(BuildError::exceeded_size_limit(size_limit));
            }
        }
        Ok(id)
    }
    fn stack_push(
        &mut self,
        nfa_id: StateID,
        epsilons: Epsilons,
    ) -> Result<(), BuildError> {
        if !self.seen.insert(nfa_id) {
            return Err(
                BuildError::not_one_pass("multiple epsilon transitions to same state"),
            );
        }
        self.stack.push((nfa_id, epsilons));
        Ok(())
    }
}
impl Iterator for LookSetIter {
    type Item = Look;
    #[inline]
    fn next(&mut self) -> Option<Look> {
        if self.set.is_empty() {
            return None;
        }
        let bit = u16::try_from(self.set.bits.trailing_zeros()).unwrap();
        let look = Look::from_repr(1 << bit)?;
        self.set = self.set.remove(look);
        Some(look)
    }
}
impl BuildError {
    fn nfa(err: crate::nfa::thompson::BuildError) -> BuildError {}
    fn word(err: UnicodeWordBoundaryError) -> BuildError {}
    fn too_many_states(limit: u64) -> BuildError {}
    fn too_many_patterns(limit: u64) -> BuildError {
        BuildError {
            kind: BuildErrorKind::TooManyPatterns {
                limit,
            },
        }
    }
    fn unsupported_look(look: Look) -> BuildError {
        BuildError {
            kind: BuildErrorKind::UnsupportedLook {
                look,
            },
        }
    }
    fn exceeded_size_limit(limit: usize) -> BuildError {}
    fn not_one_pass(msg: &'static str) -> BuildError {
        BuildError {
            kind: BuildErrorKind::NotOnePass { msg },
        }
    }
}
impl PatternEpsilons {
    const PATTERN_ID_BITS: u64 = 22;
    const PATTERN_ID_SHIFT: u64 = 64 - PatternEpsilons::PATTERN_ID_BITS;
    const PATTERN_ID_NONE: u64 = 0x00000000_003FFFFF;
    const PATTERN_ID_LIMIT: u64 = PatternEpsilons::PATTERN_ID_NONE;
    const PATTERN_ID_MASK: u64 = 0xFFFFFC00_00000000;
    const EPSILONS_MASK: u64 = 0x000003FF_FFFFFFFF;
    fn empty() -> PatternEpsilons {
        PatternEpsilons(
            PatternEpsilons::PATTERN_ID_NONE << PatternEpsilons::PATTERN_ID_SHIFT,
        )
    }
    fn is_empty(self) -> bool {}
    fn pattern_id(self) -> Option<PatternID> {}
    fn pattern_id_unchecked(self) -> PatternID {}
    fn set_pattern_id(self, pid: PatternID) -> PatternEpsilons {
        PatternEpsilons(
            (pid.as_u64() << PatternEpsilons::PATTERN_ID_SHIFT)
                | (self.0 & PatternEpsilons::EPSILONS_MASK),
        )
    }
    fn epsilons(self) -> Epsilons {}
    fn set_epsilons(self, epsilons: Epsilons) -> PatternEpsilons {
        PatternEpsilons(
            (self.0 & PatternEpsilons::PATTERN_ID_MASK)
                | (u64::from(epsilons.0) & PatternEpsilons::EPSILONS_MASK),
        )
    }
}
impl Config {
    pub fn new() -> Config {}
    pub fn match_kind(mut self, kind: MatchKind) -> Config {}
    pub fn starts_for_each_pattern(mut self, yes: bool) -> Config {}
    pub fn byte_classes(mut self, yes: bool) -> Config {}
    pub fn size_limit(mut self, limit: Option<usize>) -> Config {}
    pub fn get_match_kind(&self) -> MatchKind {}
    pub fn get_starts_for_each_pattern(&self) -> bool {
        self.starts_for_each_pattern.unwrap_or(false)
    }
    pub fn get_byte_classes(&self) -> bool {}
    pub fn get_size_limit(&self) -> Option<usize> {}
    pub(crate) fn overwrite(&self, o: Config) -> Config {}
}
impl SmallIndex {
    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    pub const MAX: SmallIndex = SmallIndex::new_unchecked(core::i32::MAX as usize - 1);
    #[cfg(target_pointer_width = "16")]
    pub const MAX: SmallIndex = SmallIndex::new_unchecked(core::isize::MAX - 1);
    pub const LIMIT: usize = SmallIndex::MAX.as_usize() + 1;
    pub const ZERO: SmallIndex = SmallIndex::new_unchecked(0);
    pub const SIZE: usize = core::mem::size_of::<SmallIndex>();
    #[inline]
    pub fn new(index: usize) -> Result<SmallIndex, SmallIndexError> {}
    #[inline]
    pub const fn new_unchecked(index: usize) -> SmallIndex {}
    #[inline]
    pub fn must(index: usize) -> SmallIndex {}
    #[inline]
    pub const fn as_usize(&self) -> usize {
        self.0 as usize
    }
    #[inline]
    pub const fn as_u64(&self) -> u64 {}
    #[inline]
    pub const fn as_u32(&self) -> u32 {}
    #[inline]
    pub const fn as_i32(&self) -> i32 {}
    #[inline]
    pub fn one_more(&self) -> usize {}
    #[inline]
    pub fn from_ne_bytes(bytes: [u8; 4]) -> Result<SmallIndex, SmallIndexError> {}
    #[inline]
    pub fn from_ne_bytes_unchecked(bytes: [u8; 4]) -> SmallIndex {}
    #[inline]
    pub fn to_ne_bytes(&self) -> [u8; 4] {}
}
impl DenseTransitions {
    #[inline]
    pub fn matches(&self, haystack: &[u8], at: usize) -> Option<StateID> {}
    #[inline]
    pub(crate) fn matches_unit(&self, unit: alphabet::Unit) -> Option<StateID> {}
    #[inline]
    pub fn matches_byte(&self, byte: u8) -> Option<StateID> {}
    pub(crate) fn iter(&self) -> impl Iterator<Item = Transition> + '_ {
        use crate::util::int::Usize;
        self.transitions
            .iter()
            .enumerate()
            .filter(|&(_, &sid)| sid != StateID::ZERO)
            .map(|(byte, &next)| Transition {
                start: byte.as_u8(),
                end: byte.as_u8(),
                next,
            })
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
    pub fn patterns(&self) -> PatternIter<'_> {
        PatternIter {
            it: PatternID::iter(self.pattern_len()),
            _marker: core::marker::PhantomData,
        }
    }
    #[inline]
    pub fn pattern_len(&self) -> usize {
        self.0.start_pattern.len()
    }
    #[inline]
    pub fn start_anchored(&self) -> StateID {
        self.0.start_anchored
    }
    #[inline]
    pub fn start_unanchored(&self) -> StateID {}
    #[inline]
    pub fn start_pattern(&self, pid: PatternID) -> Option<StateID> {
        self.0.start_pattern.get(pid.as_usize()).copied()
    }
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
    pub fn group_info(&self) -> &GroupInfo {
        &self.0.group_info()
    }
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
    pub fn look_set_any(&self) -> LookSet {
        self.0.look_set_any
    }
    #[inline]
    pub fn look_set_prefix_any(&self) -> LookSet {}
    #[inline]
    pub fn memory_usage(&self) -> usize {}
}
impl Look {
    #[inline]
    pub const fn reversed(self) -> Look {}
    #[inline]
    pub const fn as_repr(self) -> u32 {
        self as u32
    }
    #[inline]
    pub const fn from_repr(repr: u32) -> Option<Look> {}
    #[inline]
    pub const fn as_char(self) -> char {}
}
impl Epsilons {
    const SLOT_MASK: u64 = 0x000003FF_FFFFFC00;
    const SLOT_SHIFT: u64 = 10;
    const LOOK_MASK: u64 = 0x00000000_000003FF;
    fn empty() -> Epsilons {
        Epsilons(0)
    }
    fn is_empty(self) -> bool {}
    fn slots(self) -> Slots {
        Slots((self.0 >> Epsilons::SLOT_SHIFT).low_u32())
    }
    fn set_slots(self, slots: Slots) -> Epsilons {
        Epsilons(
            (u64::from(slots.0) << Epsilons::SLOT_SHIFT) | (self.0 & Epsilons::LOOK_MASK),
        )
    }
    fn looks(self) -> LookSet {
        LookSet {
            bits: (self.0 & Epsilons::LOOK_MASK).low_u32(),
        }
    }
    fn set_looks(self, look_set: LookSet) -> Epsilons {
        Epsilons(
            (self.0 & Epsilons::SLOT_MASK)
                | (u64::from(look_set.bits) & Epsilons::LOOK_MASK),
        )
    }
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
    pub(crate) fn is_empty(&self) -> bool {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn insert(&mut self, id: StateID) -> bool {}
    #[inline]
    pub(crate) fn contains(&self, id: StateID) -> bool {}
    #[inline]
    pub(crate) fn clear(&mut self) {
        self.len = 0;
    }
    #[inline]
    pub(crate) fn iter(&self) -> SparseSetIter<'_> {}
    #[inline]
    pub(crate) fn memory_usage(&self) -> usize {}
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
    pub fn is_empty(self) -> bool {}
    #[inline]
    pub fn contains(self, look: Look) -> bool {}
    #[inline]
    pub fn contains_anchor(&self) -> bool {}
    #[inline]
    pub fn contains_anchor_haystack(&self) -> bool {}
    #[inline]
    pub fn contains_anchor_line(&self) -> bool {}
    #[inline]
    pub fn contains_anchor_lf(&self) -> bool {}
    #[inline]
    pub fn contains_anchor_crlf(&self) -> bool {}
    #[inline]
    pub fn contains_word(self) -> bool {}
    #[inline]
    pub fn contains_word_unicode(self) -> bool {}
    #[inline]
    pub fn contains_word_ascii(self) -> bool {}
    #[inline]
    pub fn iter(self) -> LookSetIter {
        LookSetIter { set: self }
    }
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
    pub fn subtract(self, other: LookSet) -> LookSet {}
    #[inline]
    pub fn set_subtract(&mut self, other: LookSet) {}
    #[inline]
    pub fn union(self, other: LookSet) -> LookSet {}
    #[inline]
    pub fn set_union(&mut self, other: LookSet) {}
    #[inline]
    pub fn intersect(self, other: LookSet) -> LookSet {}
    #[inline]
    pub fn set_intersect(&mut self, other: LookSet) {}
    #[inline]
    pub fn read_repr(slice: &[u8]) -> LookSet {}
    #[inline]
    pub fn write_repr(self, slice: &mut [u8]) {}
    pub fn available(self) -> Result<(), UnicodeWordBoundaryError> {
        if self.contains_word_unicode() {
            UnicodeWordBoundaryError::check()?;
        }
        Ok(())
    }
}
impl DFA {
    fn start(&self) -> StateID {}
    fn start_pattern(&self, pid: PatternID) -> Result<StateID, MatchError> {}
    fn transition(&self, sid: StateID, byte: u8) -> Transition {}
    fn set_transition(&mut self, sid: StateID, byte: u8, to: Transition) {}
    fn sparse_transitions(&self, sid: StateID) -> SparseTransitionIter<'_> {}
    fn pattern_epsilons(&self, sid: StateID) -> PatternEpsilons {}
    fn set_pattern_epsilons(&mut self, sid: StateID, pateps: PatternEpsilons) {
        let offset = sid.as_usize() << self.stride2();
        self.table[offset + self.pateps_offset] = Transition(pateps.0);
    }
    fn prev_state_id(&self, id: StateID) -> Option<StateID> {}
    fn last_state_id(&self) -> StateID {}
    pub(super) fn swap_states(&mut self, id1: StateID, id2: StateID) {}
    pub(super) fn remap(&mut self, map: impl Fn(StateID) -> StateID) {}
}
impl GroupInfo {
    pub fn new<P, G, N>(pattern_groups: P) -> Result<GroupInfo, GroupInfoError>
    where
        P: IntoIterator<Item = G>,
        G: IntoIterator<Item = Option<N>>,
        N: AsRef<str>,
    {}
    pub fn empty() -> GroupInfo {}
    #[inline]
    pub fn to_index(&self, pid: PatternID, name: &str) -> Option<usize> {}
    #[inline]
    pub fn to_name(&self, pid: PatternID, group_index: usize) -> Option<&str> {}
    #[inline]
    pub fn pattern_names(&self, pid: PatternID) -> GroupInfoPatternNames<'_> {}
    #[inline]
    pub fn all_names(&self) -> GroupInfoAllNames<'_> {}
    #[inline]
    pub fn slots(&self, pid: PatternID, group_index: usize) -> Option<(usize, usize)> {}
    #[inline]
    pub fn slot(&self, pid: PatternID, group_index: usize) -> Option<usize> {}
    #[inline]
    pub fn pattern_len(&self) -> usize {}
    #[inline]
    pub fn group_len(&self, pid: PatternID) -> usize {}
    #[inline]
    pub fn all_group_len(&self) -> usize {}
    #[inline]
    pub fn slot_len(&self) -> usize {}
    #[inline]
    pub fn implicit_slot_len(&self) -> usize {}
    #[inline]
    pub fn explicit_slot_len(&self) -> usize {
        self.slot_len().saturating_sub(self.implicit_slot_len())
    }
    #[inline]
    pub fn memory_usage(&self) -> usize {}
}
impl Slots {
    const LIMIT: usize = 32;
    fn insert(self, slot: usize) -> Slots {
        debug_assert!(slot < Slots::LIMIT);
        Slots(self.0 | (1 << slot.as_u32()))
    }
    fn remove(self, slot: usize) -> Slots {}
    fn is_empty(self) -> bool {}
    fn iter(self) -> SlotsIter {}
    fn apply(self, at: usize, caller_explicit_slots: &mut [Option<NonMaxUsize>]) {}
}
