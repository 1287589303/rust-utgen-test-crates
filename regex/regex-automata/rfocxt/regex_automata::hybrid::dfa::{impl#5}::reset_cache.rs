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
#[derive(Debug)]
struct Lazy<'i, 'c> {
    dfa: &'i DFA,
    cache: &'c mut Cache,
}
#[derive(Clone, Debug)]
pub(crate) struct SparseSets {
    pub(crate) set1: SparseSet,
    pub(crate) set2: SparseSet,
}
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
#[derive(Clone)]
pub struct NFA(Arc<Inner>);
#[derive(Clone, Debug)]
pub struct Cache {
    /// Stack used while computing epsilon closure. This effectively lets us
    /// move what is more naturally expressed through recursion to a stack
    /// on the heap.
    stack: Vec<FollowEpsilon>,
    /// The current active states being explored for the current byte in the
    /// haystack.
    curr: ActiveStates,
    /// The next set of states we're building that will be explored for the
    /// next byte in the haystack.
    next: ActiveStates,
}
#[derive(Clone)]
pub struct DFA<T> {
    tt: Transitions<T>,
    st: StartTable<T>,
    special: Special,
    pre: Option<Prefilter>,
    quitset: ByteSet,
    flags: Flags,
}
#[derive(Clone, Debug)]
struct SearchProgress {
    start: usize,
    at: usize,
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
#[derive(Clone, Debug)]
pub struct Cache {
    /// The transition table.
    ///
    /// Given a `current` LazyStateID and an `input` byte, the next state can
    /// be computed via `trans[untagged(current) + equiv_class(input)]`. Notice
    /// that no multiplication is used. That's because state identifiers are
    /// "premultiplied."
    ///
    /// Note that the next state may be the "unknown" state. In this case, the
    /// next state is not known and determinization for `current` on `input`
    /// must be performed.
    trans: Vec<LazyStateID>,
    /// The starting states for this DFA.
    ///
    /// These are computed lazily. Initially, these are all set to "unknown"
    /// lazy state IDs.
    ///
    /// When 'starts_for_each_pattern' is disabled (the default), then the size
    /// of this is constrained to the possible starting configurations based
    /// on the search parameters. (At time of writing, that's 4.) However,
    /// when starting states for each pattern is enabled, then there are N
    /// additional groups of starting states, where each group reflects the
    /// different possible configurations and N is the number of patterns.
    starts: Vec<LazyStateID>,
    /// A sequence of NFA/DFA powerset states that have been computed for this
    /// lazy DFA. This sequence is indexable by untagged LazyStateIDs. (Every
    /// tagged LazyStateID can be used to index this sequence by converting it
    /// to its untagged form.)
    states: Vec<State>,
    /// A map from states to their corresponding IDs. This map may be accessed
    /// via the raw byte representation of a state, which means that a `State`
    /// does not need to be allocated to determine whether it already exists
    /// in this map. Indeed, the existence of such a state is what determines
    /// whether we allocate a new `State` or not.
    ///
    /// The higher level idea here is that we do just enough determinization
    /// for a state to check whether we've already computed it. If we have,
    /// then we can save a little (albeit not much) work. The real savings is
    /// in memory usage. If we never checked for trivially duplicate states,
    /// then our memory usage would explode to unreasonable levels.
    states_to_id: StateMap,
    /// Sparse sets used to track which NFA states have been visited during
    /// various traversals.
    sparses: SparseSets,
    /// Scratch space for traversing the NFA graph. (We use space on the heap
    /// instead of the call stack.)
    stack: Vec<NFAStateID>,
    /// Scratch space for building a NFA/DFA powerset state. This is used to
    /// help amortize allocation since not every powerset state generated is
    /// added to the cache. In particular, if it already exists in the cache,
    /// then there is no need to allocate a new `State` for it.
    scratch_state_builder: StateBuilderEmpty,
    /// A simple abstraction for handling the saving of at most a single state
    /// across a cache clearing. This is required for correctness. Namely, if
    /// adding a new state after clearing the cache fails, then the caller
    /// must retain the ability to continue using the state ID given. The
    /// state corresponding to the state ID is what we preserve across cache
    /// clearings.
    state_saver: StateSaver,
    /// The memory usage, in bytes, used by 'states' and 'states_to_id'. We
    /// track this as new states are added since states use a variable amount
    /// of heap. Tracking this as we add states makes it possible to compute
    /// the total amount of memory used by the determinizer in constant time.
    memory_usage_state: usize,
    /// The number of times the cache has been cleared. When a minimum cache
    /// clear count is set, then the cache will return an error instead of
    /// clearing the cache if the count has been exceeded.
    clear_count: usize,
    /// The total number of bytes searched since the last time this cache was
    /// cleared, not including the current search.
    ///
    /// This can be added to the length of the current search to get the true
    /// total number of bytes searched.
    ///
    /// This is generally only non-zero when the
    /// `Cache::search_{start,update,finish}` APIs are used to track search
    /// progress.
    bytes_searched: usize,
    /// The progress of the current search.
    ///
    /// This is only non-`None` when callers utlize the `Cache::search_start`,
    /// `Cache::search_update` and `Cache::search_finish` APIs.
    ///
    /// The purpose of recording search progress is to be able to make a
    /// determination about the efficiency of the cache. Namely, by keeping
    /// track of the
    progress: Option<SearchProgress>,
}
#[derive(Debug, Clone)]
pub struct Cache {
    pub(crate) capmatches: Captures,
    pub(crate) pikevm: wrappers::PikeVMCache,
    pub(crate) backtrack: wrappers::BoundedBacktrackerCache,
    pub(crate) onepass: wrappers::OnePassCache,
    pub(crate) hybrid: wrappers::HybridCache,
    pub(crate) revhybrid: wrappers::ReverseHybridCache,
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
#[derive(Debug, Clone)]
pub struct Cache {
    forward: dfa::Cache,
    reverse: dfa::Cache,
}
#[derive(Clone, Debug)]
pub struct Cache {
    /// Stack used on the heap for doing backtracking instead of the
    /// traditional recursive approach. We don't want recursion because then
    /// we're likely to hit a stack overflow for bigger regexes.
    stack: Vec<Frame>,
    /// The set of (StateID, HaystackOffset) pairs that have been visited
    /// by the backtracker within a single search. If such a pair has been
    /// visited, then we avoid doing the work for that pair again. This is
    /// what "bounds" the backtracking and prevents it from having worst case
    /// exponential time.
    visited: Visited,
}
#[derive(Clone, Debug)]
pub struct Cache {
    /// Scratch space used to store slots during a search. Basically, we use
    /// the caller provided slots to store slots known when a match occurs.
    /// But after a match occurs, we might continue a search but ultimately
    /// fail to extend the match. When continuing the search, we need some
    /// place to store candidate capture offsets without overwriting the slot
    /// offsets recorded for the most recently seen match.
    explicit_slots: Vec<Option<NonMaxUsize>>,
    /// The number of slots in the caller-provided 'Captures' value for the
    /// current search. This is always at most 'explicit_slots.len()', but
    /// might be less than it, if the caller provided fewer slots to fill.
    explicit_slot_len: usize,
}
#[derive(Debug)]
pub(crate) struct DFA(Option<DFAEngine>);
#[derive(Clone, Debug)]
enum StateSaver {
    /// An empty state saver. In this case, no states (other than the special
    /// sentinel states) are preserved after clearing the cache.
    None,
    /// An ID of a state (and the state itself) that should be preserved after
    /// the lazy DFA's cache has been cleared. After clearing, the updated ID
    /// is stored in 'Saved' since it may have changed.
    ToSave { id: LazyStateID, state: State },
    /// An ID that of a state that has been persisted through a lazy DFA
    /// cache clearing. The ID recorded here corresponds to an ID that was
    /// once marked as ToSave. The IDs are likely not equivalent even though
    /// the states they point to are.
    Saved(LazyStateID),
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
impl<'i, 'c> Lazy<'i, 'c> {
    fn new(dfa: &'i DFA, cache: &'c mut Cache) -> Lazy<'i, 'c> {}
    fn as_ref<'a>(&'a self) -> LazyRef<'i, 'a> {}
    #[cold]
    #[inline(never)]
    fn cache_next_state(
        &mut self,
        mut current: LazyStateID,
        unit: alphabet::Unit,
    ) -> Result<LazyStateID, CacheError> {}
    #[cold]
    #[inline(never)]
    fn cache_start_group(
        &mut self,
        anchored: Anchored,
        start: Start,
    ) -> Result<LazyStateID, StartError> {}
    fn cache_start_one(
        &mut self,
        nfa_start_id: NFAStateID,
        start: Start,
    ) -> Result<LazyStateID, CacheError> {}
    fn add_builder_state(
        &mut self,
        builder: StateBuilderNFA,
        idmap: impl Fn(LazyStateID) -> LazyStateID,
    ) -> Result<LazyStateID, CacheError> {}
    fn add_state(
        &mut self,
        state: State,
        idmap: impl Fn(LazyStateID) -> LazyStateID,
    ) -> Result<LazyStateID, CacheError> {}
    fn next_state_id(&mut self) -> Result<LazyStateID, CacheError> {}
    fn try_clear_cache(&mut self) -> Result<(), CacheError> {}
    fn reset_cache(&mut self) {
        self.cache.state_saver = StateSaver::none();
        self.clear_cache();
        self.cache.sparses.resize(self.dfa.get_nfa().states().len());
        self.cache.clear_count = 0;
        self.cache.progress = None;
    }
    fn clear_cache(&mut self) {
        self.cache.trans.clear();
        self.cache.starts.clear();
        self.cache.states.clear();
        self.cache.states_to_id.clear();
        self.cache.memory_usage_state = 0;
        self.cache.clear_count += 1;
        self.cache.bytes_searched = 0;
        if let Some(ref mut progress) = self.cache.progress {
            progress.start = progress.at;
        }
        trace!("lazy DFA cache has been cleared (count: {})", self.cache.clear_count);
        self.init_cache();
        if let Some((old_id, state)) = self.cache.state_saver.take_to_save() {
            assert!(! self.as_ref().is_sentinel(old_id), "cannot save sentinel state");
            let new_id = self
                .add_state(
                    state,
                    |id| { if old_id.is_start() { id.to_start() } else { id } },
                )
                .expect("adding one state after cache clear must work");
            self.cache.state_saver = StateSaver::Saved(new_id);
        }
    }
    fn init_cache(&mut self) {}
    fn save_state(&mut self, id: LazyStateID) {}
    fn saved_state_id(&mut self) -> LazyStateID {}
    fn set_all_transitions(&mut self, from: LazyStateID, to: LazyStateID) {}
    fn set_transition(
        &mut self,
        from: LazyStateID,
        unit: alphabet::Unit,
        to: LazyStateID,
    ) {}
    fn set_start_state(&mut self, anchored: Anchored, start: Start, id: LazyStateID) {}
    fn get_state_builder(&mut self) -> StateBuilderEmpty {}
    fn put_state_builder(&mut self, builder: StateBuilderNFA) {}
}
impl SparseSets {
    pub(crate) fn new(capacity: usize) -> SparseSets {}
    #[inline]
    pub(crate) fn resize(&mut self, new_capacity: usize) {
        self.set1.resize(new_capacity);
        self.set2.resize(new_capacity);
    }
    pub(crate) fn clear(&mut self) {}
    pub(crate) fn swap(&mut self) {}
    pub(crate) fn memory_usage(&self) -> usize {}
}
impl StateSaver {
    fn none() -> StateSaver {
        StateSaver::None
    }
    fn take_to_save(&mut self) -> Option<(LazyStateID, State)> {}
    fn take_saved(&mut self) -> Option<LazyStateID> {}
}
impl DFA {
    #[cfg(feature = "syntax")]
    pub fn new(pattern: &str) -> Result<DFA, BuildError> {}
    #[cfg(feature = "syntax")]
    pub fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<DFA, BuildError> {}
    pub fn always_match() -> Result<DFA, BuildError> {}
    pub fn never_match() -> Result<DFA, BuildError> {}
    pub fn config() -> Config {}
    pub fn builder() -> Builder {}
    pub fn create_cache(&self) -> Cache {}
    pub fn reset_cache(&self, cache: &mut Cache) {}
    pub fn pattern_len(&self) -> usize {}
    pub fn byte_classes(&self) -> &ByteClasses {}
    pub fn get_config(&self) -> &Config {}
    pub fn get_nfa(&self) -> &thompson::NFA {
        &self.nfa
    }
    fn stride2(&self) -> usize {}
    fn stride(&self) -> usize {}
    pub fn memory_usage(&self) -> usize {}
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
