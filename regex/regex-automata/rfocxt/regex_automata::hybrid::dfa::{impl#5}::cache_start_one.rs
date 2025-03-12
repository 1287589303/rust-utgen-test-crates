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
#[derive(Clone)]
pub(crate) struct StateBuilderMatches(Vec<u8>);
#[derive(Clone, Debug)]
pub(crate) struct StateBuilderEmpty(Vec<u8>);
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
#[derive(Clone)]
pub struct DFA<T> {
    tt: Transitions<T>,
    st: StartTable<T>,
    special: Special,
    pre: Option<Prefilter>,
    quitset: ByteSet,
    flags: Flags,
}
#[derive(Debug)]
pub(crate) struct DFA(Option<DFAEngine>);
#[derive(Clone)]
pub(crate) struct StateBuilderNFA {
    repr: Vec<u8>,
    prev_nfa_state_id: StateID,
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
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
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
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct LazyStateID(u32);
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
#[derive(Clone, Debug)]
pub struct CacheError(());
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
    ) -> Result<LazyStateID, CacheError> {
        let mut builder_matches = self.get_state_builder().into_matches();
        determinize::set_lookbehind_from_start(
            self.dfa.get_nfa(),
            &start,
            &mut builder_matches,
        );
        self.cache.sparses.set1.clear();
        determinize::epsilon_closure(
            self.dfa.get_nfa(),
            nfa_start_id,
            builder_matches.look_have(),
            &mut self.cache.stack,
            &mut self.cache.sparses.set1,
        );
        let mut builder = builder_matches.into_nfa();
        determinize::add_nfa_states(
            &self.dfa.get_nfa(),
            &self.cache.sparses.set1,
            &mut builder,
        );
        let tag_starts = self.dfa.get_config().get_specialize_start_states();
        self.add_builder_state(
            builder,
            |id| { if tag_starts { id.to_start() } else { id } },
        )
    }
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
    fn reset_cache(&mut self) {}
    fn clear_cache(&mut self) {}
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
    fn get_state_builder(&mut self) -> StateBuilderEmpty {
        core::mem::replace(
            &mut self.cache.scratch_state_builder,
            StateBuilderEmpty::new(),
        )
    }
    fn put_state_builder(&mut self, builder: StateBuilderNFA) {}
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
    pub fn get_config(&self) -> &Config {
        &self.config
    }
    pub fn get_nfa(&self) -> &thompson::NFA {
        &self.nfa
    }
    fn stride2(&self) -> usize {}
    fn stride(&self) -> usize {}
    pub fn memory_usage(&self) -> usize {}
}
impl Config {
    pub fn new() -> Config {}
    pub fn match_kind(mut self, kind: MatchKind) -> Config {}
    pub fn prefilter(mut self, pre: Option<Prefilter>) -> Config {}
    pub fn starts_for_each_pattern(mut self, yes: bool) -> Config {}
    pub fn byte_classes(mut self, yes: bool) -> Config {}
    pub fn unicode_word_boundary(mut self, yes: bool) -> Config {}
    pub fn quit(mut self, byte: u8, yes: bool) -> Config {}
    pub fn specialize_start_states(mut self, yes: bool) -> Config {}
    pub fn cache_capacity(mut self, bytes: usize) -> Config {}
    pub fn skip_cache_capacity_check(mut self, yes: bool) -> Config {}
    pub fn minimum_cache_clear_count(mut self, min: Option<usize>) -> Config {}
    pub fn minimum_bytes_per_state(mut self, min: Option<usize>) -> Config {}
    pub fn get_match_kind(&self) -> MatchKind {}
    pub fn get_prefilter(&self) -> Option<&Prefilter> {}
    pub fn get_starts_for_each_pattern(&self) -> bool {}
    pub fn get_byte_classes(&self) -> bool {}
    pub fn get_unicode_word_boundary(&self) -> bool {}
    pub fn get_quit(&self, byte: u8) -> bool {}
    pub fn get_specialize_start_states(&self) -> bool {
        self.specialize_start_states.unwrap_or(false)
    }
    pub fn get_cache_capacity(&self) -> usize {}
    pub fn get_skip_cache_capacity_check(&self) -> bool {}
    pub fn get_minimum_cache_clear_count(&self) -> Option<usize> {}
    pub fn get_minimum_bytes_per_state(&self) -> Option<usize> {}
    pub fn get_minimum_cache_capacity(
        &self,
        nfa: &thompson::NFA,
    ) -> Result<usize, BuildError> {}
    fn byte_classes_from_nfa(&self, nfa: &thompson::NFA, quit: &ByteSet) -> ByteClasses {}
    fn quit_set_from_nfa(&self, nfa: &thompson::NFA) -> Result<ByteSet, BuildError> {}
    fn overwrite(&self, o: Config) -> Config {}
}
impl StateBuilderMatches {
    pub(crate) fn into_nfa(mut self) -> StateBuilderNFA {
        self.repr_vec().close_match_pattern_ids();
        StateBuilderNFA {
            repr: self.0,
            prev_nfa_state_id: StateID::ZERO,
        }
    }
    pub(crate) fn set_is_from_word(&mut self) {}
    pub(crate) fn set_is_half_crlf(&mut self) {}
    pub(crate) fn look_have(&self) -> LookSet {
        LookSet::read_repr(&self.0[1..])
    }
    pub(crate) fn set_look_have(&mut self, set: impl FnMut(LookSet) -> LookSet) {}
    pub(crate) fn add_match_pattern_id(&mut self, pid: PatternID) {}
    fn repr(&self) -> Repr<'_> {}
    fn repr_vec(&mut self) -> ReprVec<'_> {}
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
pub(crate) fn set_lookbehind_from_start(
    nfa: &thompson::NFA,
    start: &Start,
    builder: &mut StateBuilderMatches,
) {
    let rev = nfa.is_reverse();
    let lineterm = nfa.look_matcher().get_line_terminator();
    let lookset = nfa.look_set_any();
    match *start {
        Start::NonWordByte => {
            if lookset.contains_word() {
                builder
                    .set_look_have(|have| {
                        have.insert(Look::WordStartHalfAscii)
                            .insert(Look::WordStartHalfUnicode)
                    });
            }
        }
        Start::WordByte => {
            if lookset.contains_word() {
                builder.set_is_from_word();
            }
        }
        Start::Text => {
            if lookset.contains_anchor_haystack() {
                builder.set_look_have(|have| have.insert(Look::Start));
            }
            if lookset.contains_anchor_line() {
                builder
                    .set_look_have(|have| {
                        have.insert(Look::StartLF).insert(Look::StartCRLF)
                    });
            }
            if lookset.contains_word() {
                builder
                    .set_look_have(|have| {
                        have.insert(Look::WordStartHalfAscii)
                            .insert(Look::WordStartHalfUnicode)
                    });
            }
        }
        Start::LineLF => {
            if rev {
                if lookset.contains_anchor_crlf() {
                    builder.set_is_half_crlf();
                }
                if lookset.contains_anchor_line() {
                    builder.set_look_have(|have| have.insert(Look::StartLF));
                }
            } else {
                if lookset.contains_anchor_line() {
                    builder.set_look_have(|have| have.insert(Look::StartCRLF));
                }
            }
            if lookset.contains_anchor_line() && lineterm == b'\n' {
                builder.set_look_have(|have| have.insert(Look::StartLF));
            }
            if lookset.contains_word() {
                builder
                    .set_look_have(|have| {
                        have.insert(Look::WordStartHalfAscii)
                            .insert(Look::WordStartHalfUnicode)
                    });
            }
        }
        Start::LineCR => {
            if lookset.contains_anchor_crlf() {
                if rev {
                    builder.set_look_have(|have| have.insert(Look::StartCRLF));
                } else {
                    builder.set_is_half_crlf();
                }
            }
            if lookset.contains_anchor_line() && lineterm == b'\r' {
                builder.set_look_have(|have| have.insert(Look::StartLF));
            }
            if lookset.contains_word() {
                builder
                    .set_look_have(|have| {
                        have.insert(Look::WordStartHalfAscii)
                            .insert(Look::WordStartHalfUnicode)
                    });
            }
        }
        Start::CustomLineTerminator => {
            if lookset.contains_anchor_line() {
                builder.set_look_have(|have| have.insert(Look::StartLF));
            }
            if lookset.contains_word() {
                if utf8::is_word_byte(lineterm) {
                    builder.set_is_from_word();
                } else {
                    builder
                        .set_look_have(|have| {
                            have.insert(Look::WordStartHalfAscii)
                                .insert(Look::WordStartHalfUnicode)
                        });
                }
            }
        }
    }
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
