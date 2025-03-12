use core::{fmt::Debug, panic::{RefUnwindSafe, UnwindSafe}};
use alloc::sync::Arc;
use regex_syntax::hir::{literal, Hir};
use crate::{
    meta::{
        error::{BuildError, RetryError, RetryFailError, RetryQuadraticError},
        regex::{Cache, RegexInfo},
        reverse_inner, wrappers,
    },
    nfa::thompson::{self, WhichCaptures, NFA},
    util::{
        captures::{Captures, GroupInfo},
        look::LookMatcher, prefilter::{self, Prefilter, PrefilterI},
        primitives::{NonMaxUsize, PatternID},
        search::{Anchored, HalfMatch, Input, Match, MatchKind, PatternSet},
    },
};
pub(super) trait Strategy: Debug + Send + Sync + RefUnwindSafe + UnwindSafe + 'static {
    fn group_info(&self) -> &GroupInfo;
    fn create_cache(&self) -> Cache;
    fn reset_cache(&self, cache: &mut Cache);
    fn is_accelerated(&self) -> bool;
    fn memory_usage(&self) -> usize;
    fn search(&self, cache: &mut Cache, input: &Input<'_>) -> Option<Match>;
    fn search_half(&self, cache: &mut Cache, input: &Input<'_>) -> Option<HalfMatch>;
    fn is_match(&self, cache: &mut Cache, input: &Input<'_>) -> bool;
    fn search_slots(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Option<PatternID>;
    fn which_overlapping_matches(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        patset: &mut PatternSet,
    );
}
#[derive(Debug)]
struct Core {
    info: RegexInfo,
    pre: Option<Prefilter>,
    nfa: NFA,
    nfarev: Option<NFA>,
    pikevm: wrappers::PikeVM,
    backtrack: wrappers::BoundedBacktracker,
    onepass: wrappers::OnePass,
    hybrid: wrappers::Hybrid,
    dfa: wrappers::DFA,
}
#[derive(Clone, Debug, Default)]
pub struct GroupInfo(Arc<GroupInfoInner>);
#[derive(Clone)]
pub struct NFA(Arc<Inner>);
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
pub struct BoundedBacktracker {
    config: Config,
    nfa: NFA,
}
#[derive(Debug)]
pub(crate) struct DFA(Option<DFAEngine>);
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
pub(crate) struct PikeVM(PikeVMEngine);
#[derive(Debug)]
pub(crate) struct Hybrid(Option<HybridEngine>);
#[derive(Clone, Debug)]
pub(crate) struct RegexInfo(Arc<RegexInfoI>);
#[derive(Debug)]
pub(crate) struct OnePass(Option<OnePassEngine>);
#[derive(Debug)]
pub(crate) struct BoundedBacktracker(Option<BoundedBacktrackerEngine>);
#[derive(Clone, Debug)]
pub struct PikeVM {
    config: Config,
    nfa: NFA,
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
pub struct DFA {
    config: Config,
    nfa: thompson::NFA,
    stride2: usize,
    start_map: StartByteMap,
    classes: ByteClasses,
    quitset: ByteSet,
    cache_capacity: usize,
}
impl Core {
    fn new(
        info: RegexInfo,
        pre: Option<Prefilter>,
        hirs: &[&Hir],
    ) -> Result<Core, BuildError> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn try_search_mayfail(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
    ) -> Option<Result<Option<Match>, RetryFailError>> {}
    fn search_nofail(&self, cache: &mut Cache, input: &Input<'_>) -> Option<Match> {}
    fn search_half_nofail(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
    ) -> Option<HalfMatch> {}
    fn search_slots_nofail(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Option<PatternID> {}
    fn is_match_nofail(&self, cache: &mut Cache, input: &Input<'_>) -> bool {}
    fn is_capture_search_needed(&self, slots_len: usize) -> bool {
        slots_len > self.nfa.group_info().implicit_slot_len()
    }
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
    pub fn implicit_slot_len(&self) -> usize {
        self.pattern_len() * 2
    }
    #[inline]
    pub fn explicit_slot_len(&self) -> usize {}
    #[inline]
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
    pub fn look_set_any(&self) -> LookSet {}
    #[inline]
    pub fn look_set_prefix_any(&self) -> LookSet {}
    #[inline]
    pub fn memory_usage(&self) -> usize {}
}
