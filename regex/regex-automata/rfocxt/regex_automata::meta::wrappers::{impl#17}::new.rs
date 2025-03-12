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
pub(crate) struct ReverseDFA(Option<ReverseDFAEngine>);
#[derive(Debug)]
pub(crate) struct ReverseDFAEngine(
    #[cfg(feature = "dfa-build")]
    dfa::dense::DFA<Vec<u32>>,
    #[cfg(not(feature = "dfa-build"))]
    (),
);
#[derive(Clone, Debug)]
pub(crate) struct RegexInfo(Arc<RegexInfoI>);
#[derive(Clone)]
pub struct NFA(Arc<Inner>);
impl ReverseDFA {
    pub(crate) fn none() -> ReverseDFA {}
    pub(crate) fn new(info: &RegexInfo, nfarev: &NFA) -> ReverseDFA {
        ReverseDFA(ReverseDFAEngine::new(info, nfarev))
    }
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn get(&self, _input: &Input<'_>) -> Option<&ReverseDFAEngine> {}
    pub(crate) fn is_some(&self) -> bool {}
    pub(crate) fn memory_usage(&self) -> usize {}
}
impl ReverseDFAEngine {
    pub(crate) fn new(info: &RegexInfo, nfarev: &NFA) -> Option<ReverseDFAEngine> {
        #[cfg(feature = "dfa-build")]
        {
            if !info.config().get_dfa() {
                return None;
            }
            if let Some(state_limit) = info.config().get_dfa_state_limit() {
                if nfarev.states().len() > state_limit {
                    debug!(
                        "skipping full reverse DFA because NFA has {} states, \
                         which exceeds the heuristic limit of {}",
                        nfarev.states().len(), state_limit,
                    );
                    return None;
                }
            }
            let size_limit = info.config().get_dfa_size_limit().map(|n| n / 2);
            let dfa_config = dfa::dense::Config::new()
                .match_kind(MatchKind::All)
                .prefilter(None)
                .accelerate(false)
                .start_kind(dfa::StartKind::Anchored)
                .starts_for_each_pattern(false)
                .byte_classes(info.config().get_byte_classes())
                .unicode_word_boundary(true)
                .specialize_start_states(false)
                .determinize_size_limit(size_limit)
                .dfa_size_limit(size_limit);
            let result = dfa::dense::Builder::new()
                .configure(dfa_config)
                .build_from_nfa(&nfarev);
            let rev = match result {
                Ok(rev) => rev,
                Err(_err) => {
                    debug!("full reverse DFA failed to build: {}", _err);
                    return None;
                }
            };
            debug!("fully compiled reverse DFA built, {} bytes", rev.memory_usage());
            Some(ReverseDFAEngine(rev))
        }
        #[cfg(not(feature = "dfa-build"))] { None }
    }
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn try_search_half_rev_limited(
        &self,
        input: &Input<'_>,
        min_start: usize,
    ) -> Result<Option<HalfMatch>, RetryError> {}
    pub(crate) fn memory_usage(&self) -> usize {}
}
