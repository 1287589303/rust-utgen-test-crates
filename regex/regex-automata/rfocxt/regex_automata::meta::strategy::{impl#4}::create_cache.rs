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
#[derive(Debug)]
pub(crate) struct BoundedBacktracker(Option<BoundedBacktrackerEngine>);
#[derive(Debug)]
pub(crate) struct Hybrid(Option<HybridEngine>);
#[derive(Clone)]
pub struct Captures {
    /// The group info that these capture groups are coupled to. This is what
    /// gives the "convenience" of the `Captures` API. Namely, it provides the
    /// slot mapping and the name|-->index mapping for capture lookups by name.
    group_info: GroupInfo,
    /// The ID of the pattern that matched. Regex engines must set this to
    /// None when no match occurs.
    pid: Option<PatternID>,
    /// The slot values, i.e., submatch offsets.
    ///
    /// In theory, the smallest sequence of slots would be something like
    /// `max(groups(pattern) for pattern in regex) * 2`, but instead, we use
    /// `sum(groups(pattern) for pattern in regex) * 2`. Why?
    ///
    /// Well, the former could be used in theory, because we don't generally
    /// have any overlapping APIs that involve capturing groups. Therefore,
    /// there's technically never any need to have slots set for multiple
    /// patterns. However, this might change some day, in which case, we would
    /// need to have slots available.
    ///
    /// The other reason is that during the execution of some regex engines,
    /// there exists a point in time where multiple slots for different
    /// patterns may be written to before knowing which pattern has matched.
    /// Therefore, the regex engines themselves, in order to support multiple
    /// patterns correctly, must have all slots available. If `Captures`
    /// doesn't have all slots available, then regex engines can't write
    /// directly into the caller provided `Captures` and must instead write
    /// into some other storage and then copy the slots involved in the match
    /// at the end of the search.
    ///
    /// So overall, at least as of the time of writing, it seems like the path
    /// of least resistance is to just require allocating all possible slots
    /// instead of the conceptual minimum. Another way to justify this is that
    /// the most common case is a single pattern, in which case, there is no
    /// inefficiency here since the 'max' and 'sum' calculations above are
    /// equivalent in that case.
    ///
    /// N.B. The mapping from group index to slot is maintained by `GroupInfo`
    /// and is considered an API guarantee. See `GroupInfo` for more details on
    /// that mapping.
    ///
    /// N.B. `Option<NonMaxUsize>` has the same size as a `usize`.
    slots: Vec<Option<NonMaxUsize>>,
}
#[derive(Debug)]
pub(crate) struct OnePass(Option<OnePassEngine>);
#[derive(Clone, Debug)]
pub(crate) struct ReverseHybridCache(
    #[cfg(feature = "hybrid")]
    Option<hybrid::dfa::Cache>,
    #[cfg(not(feature = "hybrid"))]
    (),
);
#[derive(Debug)]
pub(crate) struct PikeVM(PikeVMEngine);
#[derive(Clone)]
pub struct DFA<T> {
    tt: Transitions<T>,
    st: StartTable<T>,
    special: Special,
    pre: Option<Prefilter>,
    quitset: ByteSet,
    flags: Flags,
}
#[derive(Clone, Debug, Default)]
pub struct GroupInfo(Arc<GroupInfoInner>);
#[derive(Debug)]
pub(crate) struct DFA(Option<DFAEngine>);
#[derive(Clone, Debug)]
pub(crate) struct OnePassCache(
    #[cfg(feature = "dfa-onepass")]
    Option<onepass::Cache>,
    #[cfg(not(feature = "dfa-onepass"))]
    (),
);
#[derive(Clone, Debug)]
pub struct PikeVM {
    config: Config,
    nfa: NFA,
}
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
#[derive(Clone, Debug)]
pub struct BoundedBacktracker {
    config: Config,
    nfa: NFA,
}
#[derive(Clone, Debug)]
pub(crate) struct BoundedBacktrackerCache(
    #[cfg(feature = "nfa-backtrack")]
    Option<backtrack::Cache>,
    #[cfg(not(feature = "nfa-backtrack"))]
    (),
);
#[derive(Clone, Debug)]
pub(crate) struct PikeVMCache(Option<pikevm::Cache>);
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
#[derive(Clone, Debug)]
pub(crate) struct RegexInfo(Arc<RegexInfoI>);
#[derive(Clone)]
pub struct NFA(Arc<Inner>);
#[derive(Clone, Debug)]
pub(crate) struct HybridCache(
    #[cfg(feature = "hybrid")]
    Option<hybrid::regex::Cache>,
    #[cfg(not(feature = "hybrid"))]
    (),
);
impl Strategy for Core {
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn group_info(&self) -> &GroupInfo {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn create_cache(&self) -> Cache {
        Cache {
            capmatches: Captures::all(self.group_info().clone()),
            pikevm: self.pikevm.create_cache(),
            backtrack: self.backtrack.create_cache(),
            onepass: self.onepass.create_cache(),
            hybrid: self.hybrid.create_cache(),
            revhybrid: wrappers::ReverseHybridCache::none(),
        }
    }
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn reset_cache(&self, cache: &mut Cache) {}
    fn is_accelerated(&self) -> bool {}
    fn memory_usage(&self) -> usize {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn search(&self, cache: &mut Cache, input: &Input<'_>) -> Option<Match> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn search_half(&self, cache: &mut Cache, input: &Input<'_>) -> Option<HalfMatch> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn is_match(&self, cache: &mut Cache, input: &Input<'_>) -> bool {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn search_slots(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Option<PatternID> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn which_overlapping_matches(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        patset: &mut PatternSet,
    ) {}
}
impl BoundedBacktracker {
    pub(crate) fn new(
        info: &RegexInfo,
        pre: Option<Prefilter>,
        nfa: &NFA,
    ) -> Result<BoundedBacktracker, BuildError> {}
    pub(crate) fn create_cache(&self) -> BoundedBacktrackerCache {
        BoundedBacktrackerCache::new(self)
    }
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn get(&self, input: &Input<'_>) -> Option<&BoundedBacktrackerEngine> {}
}
impl Hybrid {
    pub(crate) fn none() -> Hybrid {}
    pub(crate) fn new(
        info: &RegexInfo,
        pre: Option<Prefilter>,
        nfa: &NFA,
        nfarev: &NFA,
    ) -> Hybrid {}
    pub(crate) fn create_cache(&self) -> HybridCache {
        HybridCache::new(self)
    }
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn get(&self, _input: &Input<'_>) -> Option<&HybridEngine> {}
    pub(crate) fn is_some(&self) -> bool {}
}
impl Captures {
    pub fn all(group_info: GroupInfo) -> Captures {
        let slots = group_info.slot_len();
        Captures {
            group_info,
            pid: None,
            slots: vec![None; slots],
        }
    }
    pub fn matches(group_info: GroupInfo) -> Captures {}
    pub fn empty(group_info: GroupInfo) -> Captures {}
    #[inline]
    pub fn is_match(&self) -> bool {}
    #[inline]
    pub fn pattern(&self) -> Option<PatternID> {}
    #[inline]
    pub fn get_match(&self) -> Option<Match> {}
    #[inline]
    pub fn get_group(&self, index: usize) -> Option<Span> {}
    pub fn get_group_by_name(&self, name: &str) -> Option<Span> {}
    pub fn iter(&self) -> CapturesPatternIter<'_> {}
    pub fn group_len(&self) -> usize {}
    pub fn group_info(&self) -> &GroupInfo {}
    pub fn interpolate_string(&self, haystack: &str, replacement: &str) -> String {}
    pub fn interpolate_string_into(
        &self,
        haystack: &str,
        replacement: &str,
        dst: &mut String,
    ) {}
    pub fn interpolate_bytes(&self, haystack: &[u8], replacement: &[u8]) -> Vec<u8> {}
    pub fn interpolate_bytes_into(
        &self,
        haystack: &[u8],
        replacement: &[u8],
        dst: &mut Vec<u8>,
    ) {}
    pub fn extract<'h, const N: usize>(
        &self,
        haystack: &'h str,
    ) -> (&'h str, [&'h str; N]) {}
    pub fn extract_bytes<'h, const N: usize>(
        &self,
        haystack: &'h [u8],
    ) -> (&'h [u8], [&'h [u8]; N]) {}
}
impl OnePass {
    pub(crate) fn new(info: &RegexInfo, nfa: &NFA) -> OnePass {}
    pub(crate) fn create_cache(&self) -> OnePassCache {
        OnePassCache::new(self)
    }
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn get(&self, input: &Input<'_>) -> Option<&OnePassEngine> {}
    pub(crate) fn memory_usage(&self) -> usize {}
}
impl ReverseHybridCache {
    pub(crate) fn none() -> ReverseHybridCache {
        #[cfg(feature = "hybrid")] { ReverseHybridCache(None) }
        #[cfg(not(feature = "hybrid"))] { ReverseHybridCache(()) }
    }
    pub(crate) fn new(builder: &ReverseHybrid) -> ReverseHybridCache {}
    pub(crate) fn reset(&mut self, builder: &ReverseHybrid) {}
    pub(crate) fn memory_usage(&self) -> usize {}
}
impl PikeVM {
    pub(crate) fn new(
        info: &RegexInfo,
        pre: Option<Prefilter>,
        nfa: &NFA,
    ) -> Result<PikeVM, BuildError> {}
    pub(crate) fn create_cache(&self) -> PikeVMCache {
        PikeVMCache::new(self)
    }
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn get(&self) -> &PikeVMEngine {}
}
