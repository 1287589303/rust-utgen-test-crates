use alloc::vec::Vec;
use crate::{
    meta::{
        error::{BuildError, RetryError, RetryFailError},
        regex::RegexInfo,
    },
    nfa::thompson::{pikevm, NFA},
    util::{prefilter::Prefilter, primitives::NonMaxUsize},
    HalfMatch, Input, Match, MatchKind, PatternID, PatternSet,
};
#[cfg(feature = "dfa-build")]
use crate::dfa;
#[cfg(feature = "dfa-onepass")]
use crate::dfa::onepass;
#[cfg(feature = "hybrid")]
use crate::hybrid;
#[cfg(feature = "nfa-backtrack")]
use crate::nfa::thompson::backtrack;
#[derive(Debug)]
pub(crate) struct Hybrid(Option<HybridEngine>);
#[derive(Debug)]
pub(crate) struct HybridEngine(
    #[cfg(feature = "hybrid")]
    hybrid::regex::Regex,
    #[cfg(not(feature = "hybrid"))]
    (),
);
#[derive(Clone, Debug)]
pub(crate) struct RegexInfo(Arc<RegexInfoI>);
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
#[derive(Clone)]
pub struct NFA(Arc<Inner>);
impl Hybrid {
    pub(crate) fn none() -> Hybrid {}
    pub(crate) fn new(
        info: &RegexInfo,
        pre: Option<Prefilter>,
        nfa: &NFA,
        nfarev: &NFA,
    ) -> Hybrid {
        Hybrid(HybridEngine::new(info, pre, nfa, nfarev))
    }
    pub(crate) fn create_cache(&self) -> HybridCache {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn get(&self, _input: &Input<'_>) -> Option<&HybridEngine> {}
    pub(crate) fn is_some(&self) -> bool {}
}
impl HybridEngine {
    pub(crate) fn new(
        info: &RegexInfo,
        pre: Option<Prefilter>,
        nfa: &NFA,
        nfarev: &NFA,
    ) -> Option<HybridEngine> {
        #[cfg(feature = "hybrid")]
        {
            if !info.config().get_hybrid() {
                return None;
            }
            let dfa_config = hybrid::dfa::Config::new()
                .match_kind(info.config().get_match_kind())
                .prefilter(pre.clone())
                .starts_for_each_pattern(true)
                .byte_classes(info.config().get_byte_classes())
                .unicode_word_boundary(true)
                .specialize_start_states(pre.is_some())
                .cache_capacity(info.config().get_hybrid_cache_capacity())
                .skip_cache_capacity_check(false)
                .minimum_cache_clear_count(Some(3))
                .minimum_bytes_per_state(Some(10));
            let result = hybrid::dfa::Builder::new()
                .configure(dfa_config.clone())
                .build_from_nfa(nfa.clone());
            let fwd = match result {
                Ok(fwd) => fwd,
                Err(_err) => {
                    debug!("forward lazy DFA failed to build: {}", _err);
                    return None;
                }
            };
            let result = hybrid::dfa::Builder::new()
                .configure(
                    dfa_config
                        .clone()
                        .match_kind(MatchKind::All)
                        .prefilter(None)
                        .specialize_start_states(false),
                )
                .build_from_nfa(nfarev.clone());
            let rev = match result {
                Ok(rev) => rev,
                Err(_err) => {
                    debug!("reverse lazy DFA failed to build: {}", _err);
                    return None;
                }
            };
            let engine = hybrid::regex::Builder::new().build_from_dfas(fwd, rev);
            debug!("lazy DFA built");
            Some(HybridEngine(engine))
        }
        #[cfg(not(feature = "hybrid"))] { None }
    }
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
    ) -> Result<(), RetryFailError> {}
}
