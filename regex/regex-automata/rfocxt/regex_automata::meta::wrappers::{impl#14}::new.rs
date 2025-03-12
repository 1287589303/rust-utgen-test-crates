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
pub(crate) struct ReverseHybrid(Option<ReverseHybridEngine>);
#[derive(Debug)]
pub(crate) struct ReverseHybridEngine(
    #[cfg(feature = "hybrid")]
    hybrid::dfa::DFA,
    #[cfg(not(feature = "hybrid"))]
    (),
);
#[derive(Clone, Debug)]
pub(crate) struct RegexInfo(Arc<RegexInfoI>);
#[derive(Clone)]
pub struct NFA(Arc<Inner>);
impl ReverseHybrid {
    pub(crate) fn none() -> ReverseHybrid {}
    pub(crate) fn new(info: &RegexInfo, nfarev: &NFA) -> ReverseHybrid {
        ReverseHybrid(ReverseHybridEngine::new(info, nfarev))
    }
    pub(crate) fn create_cache(&self) -> ReverseHybridCache {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn get(&self, _input: &Input<'_>) -> Option<&ReverseHybridEngine> {}
}
impl ReverseHybridEngine {
    pub(crate) fn new(info: &RegexInfo, nfarev: &NFA) -> Option<ReverseHybridEngine> {
        #[cfg(feature = "hybrid")]
        {
            if !info.config().get_hybrid() {
                return None;
            }
            let dfa_config = hybrid::dfa::Config::new()
                .match_kind(MatchKind::All)
                .prefilter(None)
                .starts_for_each_pattern(false)
                .byte_classes(info.config().get_byte_classes())
                .unicode_word_boundary(true)
                .specialize_start_states(false)
                .cache_capacity(info.config().get_hybrid_cache_capacity())
                .skip_cache_capacity_check(false)
                .minimum_cache_clear_count(Some(3))
                .minimum_bytes_per_state(Some(10));
            let result = hybrid::dfa::Builder::new()
                .configure(dfa_config)
                .build_from_nfa(nfarev.clone());
            let rev = match result {
                Ok(rev) => rev,
                Err(_err) => {
                    debug!("lazy reverse DFA failed to build: {}", _err);
                    return None;
                }
            };
            debug!("lazy reverse DFA built");
            Some(ReverseHybridEngine(rev))
        }
        #[cfg(not(feature = "hybrid"))] { None }
    }
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn try_search_half_rev_limited(
        &self,
        cache: &mut ReverseHybridCache,
        input: &Input<'_>,
        min_start: usize,
    ) -> Result<Option<HalfMatch>, RetryError> {}
}
