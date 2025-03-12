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
struct ReverseInner {
    core: Core,
    preinner: Prefilter,
    nfarev: NFA,
    hybrid: wrappers::ReverseHybrid,
    dfa: wrappers::ReverseDFA,
}
#[derive(Debug)]
pub(crate) struct ReverseDFAEngine(
    #[cfg(feature = "dfa-build")]
    dfa::dense::DFA<Vec<u32>>,
    #[cfg(not(feature = "dfa-build"))]
    (),
);
#[derive(Debug)]
pub(crate) struct ReverseDFA(Option<ReverseDFAEngine>);
#[derive(Debug)]
pub(crate) struct ReverseHybridEngine(
    #[cfg(feature = "hybrid")]
    hybrid::dfa::DFA,
    #[cfg(not(feature = "hybrid"))]
    (),
);
#[derive(Debug)]
pub(crate) struct ReverseHybrid(Option<ReverseHybridEngine>);
#[derive(Clone)]
pub struct NFA(Arc<Inner>);
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
pub struct Input<'h> {
    haystack: &'h [u8],
    span: Span,
    anchored: Anchored,
    earliest: bool,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct HalfMatch {
    /// The pattern ID.
    pattern: PatternID,
    /// The offset of the match.
    ///
    /// For forward searches, the offset is exclusive. For reverse searches,
    /// the offset is inclusive.
    offset: usize,
}
#[derive(Clone, Debug)]
pub(crate) struct ReverseHybridCache(
    #[cfg(feature = "hybrid")]
    Option<hybrid::dfa::Cache>,
    #[cfg(not(feature = "hybrid"))]
    (),
);
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
#[derive(Debug)]
pub(crate) enum RetryError {
    Quadratic(RetryQuadraticError),
    Fail(RetryFailError),
}
impl ReverseInner {
    fn new(core: Core, hirs: &[&Hir]) -> Result<ReverseInner, Core> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn try_search_full(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
    ) -> Result<Option<Match>, RetryError> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn try_search_half_fwd_stopat(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
    ) -> Result<Result<HalfMatch, usize>, RetryFailError> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn try_search_half_rev_limited(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        min_start: usize,
    ) -> Result<Option<HalfMatch>, RetryError> {
        if let Some(e) = self.dfa.get(&input) {
            trace!(
                "using full DFA for reverse inner search at {:?}, \
                 but will be stopped at {} to avoid quadratic behavior",
                input.get_span(), min_start,
            );
            e.try_search_half_rev_limited(&input, min_start)
        } else if let Some(e) = self.hybrid.get(&input) {
            trace!(
                "using lazy DFA for reverse inner search at {:?}, \
                 but will be stopped at {} to avoid quadratic behavior",
                input.get_span(), min_start,
            );
            e.try_search_half_rev_limited(&mut cache.revhybrid, &input, min_start)
        } else {
            unreachable!("ReverseInner always has a DFA")
        }
    }
}
impl ReverseDFAEngine {
    pub(crate) fn new(info: &RegexInfo, nfarev: &NFA) -> Option<ReverseDFAEngine> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn try_search_half_rev_limited(
        &self,
        input: &Input<'_>,
        min_start: usize,
    ) -> Result<Option<HalfMatch>, RetryError> {
        #[cfg(feature = "dfa-build")]
        {
            let dfa = &self.0;
            crate::meta::limited::dfa_try_search_half_rev(dfa, input, min_start)
        }
        #[cfg(not(feature = "dfa-build"))] { unreachable!() }
    }
    pub(crate) fn memory_usage(&self) -> usize {}
}
impl ReverseDFA {
    pub(crate) fn none() -> ReverseDFA {}
    pub(crate) fn new(info: &RegexInfo, nfarev: &NFA) -> ReverseDFA {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn get(&self, _input: &Input<'_>) -> Option<&ReverseDFAEngine> {
        let engine = self.0.as_ref()?;
        Some(engine)
    }
    pub(crate) fn is_some(&self) -> bool {}
    pub(crate) fn memory_usage(&self) -> usize {}
}
impl ReverseHybridEngine {
    pub(crate) fn new(info: &RegexInfo, nfarev: &NFA) -> Option<ReverseHybridEngine> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn try_search_half_rev_limited(
        &self,
        cache: &mut ReverseHybridCache,
        input: &Input<'_>,
        min_start: usize,
    ) -> Result<Option<HalfMatch>, RetryError> {
        #[cfg(feature = "hybrid")]
        {
            let dfa = &self.0;
            let mut cache = cache.0.as_mut().unwrap();
            crate::meta::limited::hybrid_try_search_half_rev(
                dfa,
                &mut cache,
                input,
                min_start,
            )
        }
        #[cfg(not(feature = "hybrid"))] { unreachable!() }
    }
}
impl ReverseHybrid {
    pub(crate) fn none() -> ReverseHybrid {}
    pub(crate) fn new(info: &RegexInfo, nfarev: &NFA) -> ReverseHybrid {}
    pub(crate) fn create_cache(&self) -> ReverseHybridCache {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn get(&self, _input: &Input<'_>) -> Option<&ReverseHybridEngine> {
        let engine = self.0.as_ref()?;
        Some(engine)
    }
}
