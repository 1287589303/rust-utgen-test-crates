#[cfg(feature = "std")]
type StateMap = std::collections::HashMap<State, StateID>;
#[cfg(not(feature = "std"))]
type StateMap = BTreeMap<State, StateID>;
use alloc::{collections::BTreeMap, vec::Vec};
use crate::{
    dfa::{
        dense::{self, BuildError},
        DEAD,
    },
    nfa::thompson,
    util::{
        self, alphabet::{self, ByteSet},
        determinize::{State, StateBuilderEmpty, StateBuilderNFA},
        primitives::{PatternID, StateID},
        search::{Anchored, MatchKind},
        sparse_set::SparseSets, start::Start,
    },
};
#[derive(Debug)]
struct Runner<'a> {
    /// The configuration used to initialize determinization.
    config: Config,
    /// The NFA we're converting into a DFA.
    nfa: &'a thompson::NFA,
    /// The DFA we're building.
    dfa: &'a mut dense::OwnedDFA,
    /// Each DFA state being built is defined as an *ordered* set of NFA
    /// states, along with some meta facts about the ordered set of NFA states.
    ///
    /// This is never empty. The first state is always a dummy state such that
    /// a state id == 0 corresponds to a dead state. The second state is always
    /// the quit state.
    ///
    /// Why do we have states in both a `Vec` and in a cache map below?
    /// Well, they serve two different roles based on access patterns.
    /// `builder_states` is the canonical home of each state, and provides
    /// constant random access by a DFA state's ID. The cache map below, on
    /// the other hand, provides a quick way of searching for identical DFA
    /// states by using the DFA state as a key in the map. Of course, we use
    /// reference counting to avoid actually duplicating the state's data
    /// itself. (Although this has never been benchmarked.) Note that the cache
    /// map does not give us full minimization; it just lets us avoid some very
    /// obvious redundant states.
    ///
    /// Note that the index into this Vec isn't quite the DFA's state ID.
    /// Rather, it's just an index. To get the state ID, you have to multiply
    /// it by the DFA's stride. That's done by self.dfa.from_index. And the
    /// inverse is self.dfa.to_index.
    ///
    /// Moreover, DFA states don't usually retain the IDs assigned to them
    /// by their position in this Vec. After determinization completes,
    /// states are shuffled around to support other optimizations. See the
    /// sibling 'special' module for more details on that. (The reason for
    /// mentioning this is that if you print out the DFA for debugging during
    /// determinization, and then print out the final DFA after it is fully
    /// built, then the state IDs likely won't match up.)
    builder_states: Vec<State>,
    /// A cache of DFA states that already exist and can be easily looked up
    /// via ordered sets of NFA states.
    ///
    /// See `builder_states` docs for why we store states in two different
    /// ways.
    cache: StateMap,
    /// The memory usage, in bytes, used by builder_states and cache. We track
    /// this as new states are added since states use a variable amount of
    /// heap. Tracking this as we add states makes it possible to compute the
    /// total amount of memory used by the determinizer in constant time.
    memory_usage_state: usize,
    /// A pair of sparse sets for tracking ordered sets of NFA state IDs.
    /// These are reused throughout determinization. A bounded sparse set
    /// gives us constant time insertion, membership testing and clearing.
    sparses: SparseSets,
    /// Scratch space for a stack of NFA states to visit, for depth first
    /// visiting without recursion.
    stack: Vec<StateID>,
    /// Scratch space for storing an ordered sequence of NFA states, for
    /// amortizing allocation. This is principally useful for when we avoid
    /// adding a new DFA state since it already exists. In order to detect this
    /// case though, we still need an ordered set of NFA state IDs. So we use
    /// this space to stage that ordered set before we know whether we need to
    /// create a new DFA state or not.
    scratch_state_builder: StateBuilderEmpty,
}
#[derive(Clone)]
pub struct NFA(Arc<Inner>);
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
pub(crate) struct State<'a> {
    id: StateID,
    stride2: usize,
    transitions: &'a [StateID],
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
#[derive(Clone, Debug)]
pub struct Config {
    look_behind: Option<u8>,
    anchored: Anchored,
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
#[derive(Clone, Debug)]
pub(crate) struct Config {
    match_kind: MatchKind,
    quit: ByteSet,
    dfa_size_limit: Option<usize>,
    determinize_size_limit: Option<usize>,
}
#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub(crate) struct State(Arc<[u8]>);
#[derive(Clone, Debug)]
pub(crate) struct SparseSets {
    pub(crate) set1: SparseSet,
    pub(crate) set2: SparseSet,
}
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
#[derive(Clone, Debug)]
pub(crate) struct StateBuilderEmpty(Vec<u8>);
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
#[derive(Clone, Default)]
struct State {
    transitions: Vec<Transition>,
    chunks: Vec<(usize, usize)>,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
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
#[derive(Clone, Debug, Default)]
pub struct Config {
    match_kind: Option<MatchKind>,
    pre: Option<Option<Prefilter>>,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
#[cfg(feature = "dfa-build")]
#[derive(Clone, Debug)]
pub struct BuildError {
    kind: BuildErrorKind,
}
#[derive(Clone)]
struct State {
    /// A sorted sequence of non-overlapping transitions to other states. Each
    /// transition corresponds to a single range of bytes.
    transitions: Vec<Transition>,
}
#[derive(Clone, Debug, Default)]
pub struct Config {
    match_kind: Option<MatchKind>,
    starts_for_each_pattern: Option<bool>,
    byte_classes: Option<bool>,
    size_limit: Option<Option<usize>>,
}
#[derive(Clone, Debug, Default)]
pub struct Config {
    pre: Option<Option<Prefilter>>,
    visited_capacity: Option<usize>,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Anchored {
    /// Run an unanchored search. This means a match may occur anywhere at or
    /// after the start position of the search.
    ///
    /// This search can return a match for any pattern in the regex.
    No,
    /// Run an anchored search. This means that a match must begin at the
    /// start position of the search.
    ///
    /// This search can return a match for any pattern in the regex.
    Yes,
    /// Run an anchored search for a specific pattern. This means that a match
    /// must be for the given pattern and must begin at the start position of
    /// the search.
    Pattern(PatternID),
}
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
impl<'a> Runner<'a> {
    fn run(mut self) -> Result<(), BuildError> {}
    fn cached_state(
        &mut self,
        dfa_id: StateID,
        unit: alphabet::Unit,
    ) -> Result<(StateID, bool), BuildError> {}
    fn add_all_starts(
        &mut self,
        dfa_state_ids: &mut Vec<StateID>,
    ) -> Result<(), BuildError> {}
    fn add_start_group(
        &mut self,
        anchored: Anchored,
        dfa_state_ids: &mut Vec<StateID>,
    ) -> Result<(), BuildError> {
        let nfa_start = match anchored {
            Anchored::No => self.nfa.start_unanchored(),
            Anchored::Yes => self.nfa.start_anchored(),
            Anchored::Pattern(pid) => {
                self.nfa.start_pattern(pid).expect("valid pattern ID")
            }
        };
        let (id, is_new) = self.add_one_start(nfa_start, Start::NonWordByte)?;
        self.dfa.set_start_state(anchored, Start::NonWordByte, id);
        if is_new {
            dfa_state_ids.push(id);
        }
        if !self.nfa.look_set_prefix_any().contains_word() {
            self.dfa.set_start_state(anchored, Start::WordByte, id);
        } else {
            let (id, is_new) = self.add_one_start(nfa_start, Start::WordByte)?;
            self.dfa.set_start_state(anchored, Start::WordByte, id);
            if is_new {
                dfa_state_ids.push(id);
            }
        }
        if !self.nfa.look_set_prefix_any().contains_anchor() {
            self.dfa.set_start_state(anchored, Start::Text, id);
            self.dfa.set_start_state(anchored, Start::LineLF, id);
            self.dfa.set_start_state(anchored, Start::LineCR, id);
            self.dfa.set_start_state(anchored, Start::CustomLineTerminator, id);
        } else {
            let (id, is_new) = self.add_one_start(nfa_start, Start::Text)?;
            self.dfa.set_start_state(anchored, Start::Text, id);
            if is_new {
                dfa_state_ids.push(id);
            }
            let (id, is_new) = self.add_one_start(nfa_start, Start::LineLF)?;
            self.dfa.set_start_state(anchored, Start::LineLF, id);
            if is_new {
                dfa_state_ids.push(id);
            }
            let (id, is_new) = self.add_one_start(nfa_start, Start::LineCR)?;
            self.dfa.set_start_state(anchored, Start::LineCR, id);
            if is_new {
                dfa_state_ids.push(id);
            }
            let (id, is_new) = self
                .add_one_start(nfa_start, Start::CustomLineTerminator)?;
            self.dfa.set_start_state(anchored, Start::CustomLineTerminator, id);
            if is_new {
                dfa_state_ids.push(id);
            }
        }
        Ok(())
    }
    fn add_one_start(
        &mut self,
        nfa_start: StateID,
        start: Start,
    ) -> Result<(StateID, bool), BuildError> {
        let mut builder_matches = self.get_state_builder().into_matches();
        util::determinize::set_lookbehind_from_start(
            self.nfa,
            &start,
            &mut builder_matches,
        );
        self.sparses.set1.clear();
        util::determinize::epsilon_closure(
            self.nfa,
            nfa_start,
            builder_matches.look_have(),
            &mut self.stack,
            &mut self.sparses.set1,
        );
        let mut builder = builder_matches.into_nfa();
        util::determinize::add_nfa_states(&self.nfa, &self.sparses.set1, &mut builder);
        self.maybe_add_state(builder)
    }
    fn maybe_add_state(
        &mut self,
        builder: StateBuilderNFA,
    ) -> Result<(StateID, bool), BuildError> {}
    fn add_state(&mut self, builder: StateBuilderNFA) -> Result<StateID, BuildError> {}
    fn get_state_builder(&mut self) -> StateBuilderEmpty {}
    fn put_state_builder(&mut self, builder: StateBuilderNFA) {}
    fn memory_usage(&self) -> usize {}
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
    pub fn start_anchored(&self) -> StateID {
        self.0.start_anchored
    }
    #[inline]
    pub fn start_unanchored(&self) -> StateID {
        self.0.start_unanchored
    }
    #[inline]
    pub fn start_pattern(&self, pid: PatternID) -> Option<StateID> {
        self.0.start_pattern.get(pid.as_usize()).copied()
    }
    #[inline]
    pub(crate) fn byte_class_set(&self) -> &ByteClassSet {}
    #[inline]
    pub fn byte_classes(&self) -> &ByteClasses {}
    #[inline]
    pub fn state(&self, id: StateID) -> &State {}
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
    pub fn is_reverse(&self) -> bool {}
    #[inline]
    pub fn is_always_start_anchored(&self) -> bool {}
    #[inline]
    pub fn look_matcher(&self) -> &LookMatcher {}
    #[inline]
    pub fn look_set_any(&self) -> LookSet {}
    #[inline]
    pub fn look_set_prefix_any(&self) -> LookSet {
        self.0.look_set_prefix_any
    }
    #[inline]
    pub fn memory_usage(&self) -> usize {}
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
    pub fn contains_anchor(&self) -> bool {
        self.contains_anchor_haystack() || self.contains_anchor_line()
    }
    #[inline]
    pub fn contains_anchor_haystack(&self) -> bool {}
    #[inline]
    pub fn contains_anchor_line(&self) -> bool {}
    #[inline]
    pub fn contains_anchor_lf(&self) -> bool {}
    #[inline]
    pub fn contains_anchor_crlf(&self) -> bool {}
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
    pub fn insert(self, look: Look) -> LookSet {}
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
    pub fn available(self) -> Result<(), UnicodeWordBoundaryError> {}
}
