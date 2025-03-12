use core::{fmt, mem};
use alloc::{boxed::Box, format, string::String, sync::Arc, vec, vec::Vec};
#[cfg(feature = "syntax")]
use crate::nfa::thompson::{
    compiler::{Compiler, Config},
    error::BuildError,
};
use crate::{
    nfa::thompson::builder::Builder,
    util::{
        alphabet::{self, ByteClassSet, ByteClasses},
        captures::{GroupInfo, GroupInfoError},
        look::{Look, LookMatcher, LookSet},
        primitives::{IteratorIndexExt, PatternID, PatternIDIter, SmallIndex, StateID},
        sparse_set::SparseSet,
    },
};
#[derive(Default)]
pub(super) struct Inner {
    /// The state sequence. This sequence is guaranteed to be indexable by all
    /// starting state IDs, and it is also guaranteed to contain at most one
    /// `Match` state for each pattern compiled into this NFA. (A pattern may
    /// not have a corresponding `Match` state if a `Match` state is impossible
    /// to reach.)
    states: Vec<State>,
    /// The anchored starting state of this NFA.
    start_anchored: StateID,
    /// The unanchored starting state of this NFA.
    start_unanchored: StateID,
    /// The starting states for each individual pattern. Starting at any
    /// of these states will result in only an anchored search for the
    /// corresponding pattern. The vec is indexed by pattern ID. When the NFA
    /// contains a single regex, then `start_pattern[0]` and `start_anchored`
    /// are always equivalent.
    start_pattern: Vec<StateID>,
    /// Info about the capturing groups in this NFA. This is responsible for
    /// mapping groups to slots, mapping groups to names and names to groups.
    group_info: GroupInfo,
    /// A representation of equivalence classes over the transitions in this
    /// NFA. Two bytes in the same equivalence class must not discriminate
    /// between a match or a non-match. This map can be used to shrink the
    /// total size of a DFA's transition table with a small match-time cost.
    ///
    /// Note that the NFA's transitions are *not* defined in terms of these
    /// equivalence classes. The NFA's transitions are defined on the original
    /// byte values. For the most part, this is because they wouldn't really
    /// help the NFA much since the NFA already uses a sparse representation
    /// to represent transitions. Byte classes are most effective in a dense
    /// representation.
    byte_class_set: ByteClassSet,
    /// This is generated from `byte_class_set`, and essentially represents the
    /// same thing but supports different access patterns. Namely, this permits
    /// looking up the equivalence class of a byte very cheaply.
    ///
    /// Ideally we would just store this, but because of annoying code
    /// structure reasons, we keep both this and `byte_class_set` around for
    /// now. I think I would prefer that `byte_class_set` were computed in the
    /// `Builder`, but right now, we compute it as states are added to the
    /// `NFA`.
    byte_classes: ByteClasses,
    /// Whether this NFA has a `Capture` state anywhere.
    has_capture: bool,
    /// When the empty string is in the language matched by this NFA.
    has_empty: bool,
    /// Whether UTF-8 mode is enabled for this NFA. Briefly, this means that
    /// all non-empty matches produced by this NFA correspond to spans of valid
    /// UTF-8, and any empty matches produced by this NFA that split a UTF-8
    /// encoded codepoint should be filtered out by the corresponding regex
    /// engine.
    utf8: bool,
    /// Whether this NFA is meant to be matched in reverse or not.
    reverse: bool,
    /// The matcher to be used for look-around assertions.
    look_matcher: LookMatcher,
    /// The union of all look-around assertions that occur anywhere within
    /// this NFA. If this set is empty, then it means there are precisely zero
    /// conditional epsilon transitions in the NFA.
    look_set_any: LookSet,
    /// The union of all look-around assertions that occur as a zero-length
    /// prefix for any of the patterns in this NFA.
    look_set_prefix_any: LookSet,
    /// Heap memory used indirectly by NFA states and other things (like the
    /// various capturing group representations above). Since each state
    /// might use a different amount of heap, we need to keep track of this
    /// incrementally.
    memory_extra: usize,
}
#[derive(Clone)]
struct State {
    /// A sorted sequence of non-overlapping transitions to other states. Each
    /// transition corresponds to a single range of bytes.
    transitions: Vec<Transition>,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub(crate) struct State(Arc<[u8]>);
pub(crate) struct State<'a> {
    id: StateID,
    stride2: usize,
    transitions: &'a [StateID],
}
#[derive(Clone, Copy)]
pub struct ByteClasses([u8; 256]);
#[cfg(feature = "alloc")]
#[derive(Clone, Debug)]
pub(crate) struct ByteClassSet(ByteSet);
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
#[derive(Clone, Default)]
struct State {
    transitions: Vec<Transition>,
    chunks: Vec<(usize, usize)>,
}
#[derive(Clone, Debug, Default)]
pub struct GroupInfo(Arc<GroupInfoInner>);
#[derive(Clone, Debug)]
pub struct LookMatcher {
    lineterm: DebugByte,
}
#[derive(Clone, Debug)]
pub struct GroupInfoError {
    kind: GroupInfoErrorKind,
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
#[derive(Clone, Debug, Eq, PartialEq)]
enum State {
    /// An empty state whose only purpose is to forward the automaton to
    /// another state via an unconditional epsilon transition.
    ///
    /// Unconditional epsilon transitions are quite useful during the
    /// construction of an NFA, as they permit the insertion of no-op
    /// placeholders that make it easier to compose NFA sub-graphs. When
    /// the Thompson NFA builder produces a final NFA, all unconditional
    /// epsilon transitions are removed, and state identifiers are remapped
    /// accordingly.
    Empty {
        /// The next state that this state should transition to.
        next: StateID,
    },
    /// A state that only transitions to another state if the current input
    /// byte is in a particular range of bytes.
    ByteRange { trans: Transition },
    /// A state with possibly many transitions, represented in a sparse
    /// fashion. Transitions must be ordered lexicographically by input range
    /// and be non-overlapping. As such, this may only be used when every
    /// transition has equal priority. (In practice, this is only used for
    /// encoding large UTF-8 automata.) In contrast, a `Union` state has each
    /// alternate in order of priority. Priority is used to implement greedy
    /// matching and also alternations themselves, e.g., `abc|a` where `abc`
    /// has priority over `a`.
    ///
    /// To clarify, it is possible to remove `Sparse` and represent all things
    /// that `Sparse` is used for via `Union`. But this creates a more bloated
    /// NFA with more epsilon transitions than is necessary in the special case
    /// of character classes.
    Sparse { transitions: Vec<Transition> },
    /// A conditional epsilon transition satisfied via some sort of
    /// look-around.
    Look { look: Look, next: StateID },
    /// An empty state that records the start of a capture location. This is an
    /// unconditional epsilon transition like `Empty`, except it can be used to
    /// record position information for a capture group when using the NFA for
    /// search.
    CaptureStart {
        /// The ID of the pattern that this capture was defined.
        pattern_id: PatternID,
        /// The capture group index that this capture state corresponds to.
        /// The capture group index is always relative to its corresponding
        /// pattern. Therefore, in the presence of multiple patterns, both the
        /// pattern ID and the capture group index are required to uniquely
        /// identify a capturing group.
        group_index: SmallIndex,
        /// The next state that this state should transition to.
        next: StateID,
    },
    /// An empty state that records the end of a capture location. This is an
    /// unconditional epsilon transition like `Empty`, except it can be used to
    /// record position information for a capture group when using the NFA for
    /// search.
    CaptureEnd {
        /// The ID of the pattern that this capture was defined.
        pattern_id: PatternID,
        /// The capture group index that this capture state corresponds to.
        /// The capture group index is always relative to its corresponding
        /// pattern. Therefore, in the presence of multiple patterns, both the
        /// pattern ID and the capture group index are required to uniquely
        /// identify a capturing group.
        group_index: SmallIndex,
        /// The next state that this state should transition to.
        next: StateID,
    },
    /// An alternation such that there exists an epsilon transition to all
    /// states in `alternates`, where matches found via earlier transitions
    /// are preferred over later transitions.
    Union { alternates: Vec<StateID> },
    /// An alternation such that there exists an epsilon transition to all
    /// states in `alternates`, where matches found via later transitions are
    /// preferred over earlier transitions.
    ///
    /// This "reverse" state exists for convenience during compilation that
    /// permits easy construction of non-greedy combinations of NFA states. At
    /// the end of compilation, Union and UnionReverse states are merged into
    /// one Union type of state, where the latter has its epsilon transitions
    /// reversed to reflect the priority inversion.
    ///
    /// The "convenience" here arises from the fact that as new states are
    /// added to the list of `alternates`, we would like that add operation
    /// to be amortized constant time. But if we used a `Union`, we'd need to
    /// prepend the state, which takes O(n) time. There are other approaches we
    /// could use to solve this, but this seems simple enough.
    UnionReverse { alternates: Vec<StateID> },
    /// A state that cannot be transitioned out of. This is useful for cases
    /// where you want to prevent matching from occurring. For example, if your
    /// regex parser permits empty character classes, then one could choose a
    /// `Fail` state to represent it.
    Fail,
    /// A match state. There is at most one such occurrence of this state in
    /// an NFA for each pattern compiled into the NFA. At time of writing, a
    /// match state is always produced for every pattern given, but in theory,
    /// if a pattern can never lead to a match, then the match state could be
    /// omitted.
    ///
    /// `pattern_id` refers to the ID of the pattern itself, which corresponds
    /// to the pattern's index (starting at 0).
    Match { pattern_id: PatternID },
}
impl Inner {
    pub(super) fn into_nfa(mut self) -> NFA {}
    pub(super) fn group_info(&self) -> &GroupInfo {}
    pub(super) fn add(&mut self, state: State) -> StateID {}
    pub(super) fn set_starts(
        &mut self,
        start_anchored: StateID,
        start_unanchored: StateID,
        start_pattern: &[StateID],
    ) {}
    pub(super) fn set_utf8(&mut self, yes: bool) {}
    pub(super) fn set_reverse(&mut self, yes: bool) {}
    pub(super) fn set_look_matcher(&mut self, m: LookMatcher) {}
    pub(super) fn set_captures(
        &mut self,
        captures: &[Vec<Option<Arc<str>>>],
    ) -> Result<(), GroupInfoError> {
        self.group_info = GroupInfo::new(
            captures.iter().map(|x| x.iter().map(|y| y.as_ref())),
        )?;
        Ok(())
    }
    pub(super) fn remap(&mut self, old_to_new: &[StateID]) {}
}
