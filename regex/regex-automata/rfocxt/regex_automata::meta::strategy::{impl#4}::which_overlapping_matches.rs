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
pub(crate) struct DFA(Option<DFAEngine>);
#[derive(Debug)]
pub(crate) struct DFAEngine(
    #[cfg(feature = "dfa-build")]
    dfa::regex::Regex,
    #[cfg(not(feature = "dfa-build"))]
    (),
);
#[derive(Debug)]
pub(crate) struct HybridEngine(
    #[cfg(feature = "hybrid")]
    hybrid::regex::Regex,
    #[cfg(not(feature = "hybrid"))]
    (),
);
#[derive(Debug)]
pub(crate) struct PikeVM(PikeVMEngine);
#[derive(Debug)]
pub(crate) struct PikeVMEngine(pikevm::PikeVM);
#[derive(Debug)]
pub(crate) struct Hybrid(Option<HybridEngine>);
#[derive(Debug)]
pub(crate) struct RetryFailError {
    offset: usize,
}
#[derive(Debug)]
pub(crate) struct OnePass(Option<OnePassEngine>);
#[cfg(feature = "alloc")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PatternSet {
    /// The number of patterns set to 'true' in this set.
    len: usize,
    /// A map from PatternID to boolean of whether a pattern matches or not.
    ///
    /// This should probably be a bitset, but it's probably unlikely to matter
    /// much in practice.
    ///
    /// The main downside of this representation (and similarly for a bitset)
    /// is that iteration scales with the capacity of the set instead of
    /// the length of the set. This doesn't seem likely to be a problem in
    /// practice.
    ///
    /// Another alternative is to just use a 'SparseSet' for this. It does use
    /// more memory (quite a bit more), but that seems fine I think compared
    /// to the memory being used by the regex engine. The real hiccup with
    /// it is that it yields pattern IDs in the order they were inserted.
    /// Which is actually kind of nice, but at the time of writing, pattern
    /// IDs are yielded in ascending order in the regex crate RegexSet API.
    /// If we did change to 'SparseSet', we could provide an additional
    /// 'iter_match_order' iterator, but keep the ascending order one for
    /// compatibility.
    which: alloc::boxed::Box<[bool]>,
}
#[derive(Clone)]
pub struct Input<'h> {
    haystack: &'h [u8],
    span: Span,
    anchored: Anchored,
    earliest: bool,
}
#[derive(Debug)]
pub(crate) struct BoundedBacktracker(Option<BoundedBacktrackerEngine>);
#[derive(Clone)]
pub struct DFA<T> {
    tt: Transitions<T>,
    st: StartTable<T>,
    special: Special,
    pre: Option<Prefilter>,
    quitset: ByteSet,
    flags: Flags,
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
#[derive(Clone, Debug)]
pub(crate) struct PikeVMCache(Option<pikevm::Cache>);
#[derive(Clone, Debug)]
pub struct BoundedBacktracker {
    config: Config,
    nfa: NFA,
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
pub struct PikeVM {
    config: Config,
    nfa: NFA,
}
#[derive(Clone, Debug)]
pub(crate) struct HybridCache(
    #[cfg(feature = "hybrid")]
    Option<hybrid::regex::Cache>,
    #[cfg(not(feature = "hybrid"))]
    (),
);
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
pub(crate) struct RegexInfo(Arc<RegexInfoI>);
#[derive(Clone)]
pub struct NFA(Arc<Inner>);
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
impl Strategy for Core {
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn group_info(&self) -> &GroupInfo {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn create_cache(&self) -> Cache {}
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
    ) {
        if let Some(e) = self.dfa.get(input) {
            trace!("using full DFA for overlapping search at {:?}", input.get_span());
            let _err = match e.try_which_overlapping_matches(input, patset) {
                Ok(()) => return,
                Err(err) => err,
            };
            trace!("fast overlapping search failed: {}", _err);
        } else if let Some(e) = self.hybrid.get(input) {
            trace!("using lazy DFA for overlapping search at {:?}", input.get_span());
            let _err = match e
                .try_which_overlapping_matches(&mut cache.hybrid, input, patset)
            {
                Ok(()) => {
                    return;
                }
                Err(err) => err,
            };
            trace!("fast overlapping search failed: {}", _err);
        }
        trace!("using PikeVM for overlapping search at {:?}", input.get_span());
        let e = self.pikevm.get();
        e.which_overlapping_matches(&mut cache.pikevm, input, patset)
    }
}
impl DFA {
    pub(crate) fn none() -> DFA {}
    pub(crate) fn new(
        info: &RegexInfo,
        pre: Option<Prefilter>,
        nfa: &NFA,
        nfarev: &NFA,
    ) -> DFA {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn get(&self, _input: &Input<'_>) -> Option<&DFAEngine> {
        let engine = self.0.as_ref()?;
        Some(engine)
    }
    pub(crate) fn is_some(&self) -> bool {}
    pub(crate) fn memory_usage(&self) -> usize {}
}
impl DFAEngine {
    pub(crate) fn new(
        info: &RegexInfo,
        pre: Option<Prefilter>,
        nfa: &NFA,
        nfarev: &NFA,
    ) -> Option<DFAEngine> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn try_search(
        &self,
        input: &Input<'_>,
    ) -> Result<Option<Match>, RetryFailError> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn try_search_half_fwd(
        &self,
        input: &Input<'_>,
    ) -> Result<Option<HalfMatch>, RetryFailError> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn try_search_half_fwd_stopat(
        &self,
        input: &Input<'_>,
    ) -> Result<Result<HalfMatch, usize>, RetryFailError> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn try_search_half_rev(
        &self,
        input: &Input<'_>,
    ) -> Result<Option<HalfMatch>, RetryFailError> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn try_search_half_rev_limited(
        &self,
        input: &Input<'_>,
        min_start: usize,
    ) -> Result<Option<HalfMatch>, RetryError> {}
    #[inline]
    pub(crate) fn try_which_overlapping_matches(
        &self,
        input: &Input<'_>,
        patset: &mut PatternSet,
    ) -> Result<(), RetryFailError> {
        #[cfg(feature = "dfa-build")]
        {
            use crate::dfa::Automaton;
            self.0
                .forward()
                .try_which_overlapping_matches(input, patset)
                .map_err(|e| e.into())
        }
        #[cfg(not(feature = "dfa-build"))] { unreachable!() }
    }
    pub(crate) fn memory_usage(&self) -> usize {}
}
impl HybridEngine {
    pub(crate) fn new(
        info: &RegexInfo,
        pre: Option<Prefilter>,
        nfa: &NFA,
        nfarev: &NFA,
    ) -> Option<HybridEngine> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn try_search(
        &self,
        cache: &mut HybridCache,
        input: &Input<'_>,
    ) -> Result<Option<Match>, RetryFailError> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn try_search_half_fwd(
        &self,
        cache: &mut HybridCache,
        input: &Input<'_>,
    ) -> Result<Option<HalfMatch>, RetryFailError> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn try_search_half_fwd_stopat(
        &self,
        cache: &mut HybridCache,
        input: &Input<'_>,
    ) -> Result<Result<HalfMatch, usize>, RetryFailError> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn try_search_half_rev(
        &self,
        cache: &mut HybridCache,
        input: &Input<'_>,
    ) -> Result<Option<HalfMatch>, RetryFailError> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn try_search_half_rev_limited(
        &self,
        cache: &mut HybridCache,
        input: &Input<'_>,
        min_start: usize,
    ) -> Result<Option<HalfMatch>, RetryError> {}
    #[inline]
    pub(crate) fn try_which_overlapping_matches(
        &self,
        cache: &mut HybridCache,
        input: &Input<'_>,
        patset: &mut PatternSet,
    ) -> Result<(), RetryFailError> {
        #[cfg(feature = "hybrid")]
        {
            let fwd = self.0.forward();
            let mut fwdcache = cache.0.as_mut().unwrap().as_parts_mut().0;
            fwd.try_which_overlapping_matches(&mut fwdcache, input, patset)
                .map_err(|e| e.into())
        }
        #[cfg(not(feature = "hybrid"))] { unreachable!() }
    }
}
impl PikeVM {
    pub(crate) fn new(
        info: &RegexInfo,
        pre: Option<Prefilter>,
        nfa: &NFA,
    ) -> Result<PikeVM, BuildError> {}
    pub(crate) fn create_cache(&self) -> PikeVMCache {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn get(&self) -> &PikeVMEngine {
        &self.0
    }
}
impl PikeVMEngine {
    pub(crate) fn new(
        info: &RegexInfo,
        pre: Option<Prefilter>,
        nfa: &NFA,
    ) -> Result<PikeVMEngine, BuildError> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn is_match(&self, cache: &mut PikeVMCache, input: &Input<'_>) -> bool {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn search_slots(
        &self,
        cache: &mut PikeVMCache,
        input: &Input<'_>,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Option<PatternID> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn which_overlapping_matches(
        &self,
        cache: &mut PikeVMCache,
        input: &Input<'_>,
        patset: &mut PatternSet,
    ) {
        self.0.which_overlapping_matches(cache.0.as_mut().unwrap(), input, patset)
    }
}
impl Hybrid {
    pub(crate) fn none() -> Hybrid {}
    pub(crate) fn new(
        info: &RegexInfo,
        pre: Option<Prefilter>,
        nfa: &NFA,
        nfarev: &NFA,
    ) -> Hybrid {}
    pub(crate) fn create_cache(&self) -> HybridCache {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn get(&self, _input: &Input<'_>) -> Option<&HybridEngine> {
        let engine = self.0.as_ref()?;
        Some(engine)
    }
    pub(crate) fn is_some(&self) -> bool {}
}
