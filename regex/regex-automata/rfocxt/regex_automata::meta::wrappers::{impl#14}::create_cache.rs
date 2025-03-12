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
#[derive(Clone, Debug)]
pub(crate) struct ReverseHybridCache(
    #[cfg(feature = "hybrid")]
    Option<hybrid::dfa::Cache>,
    #[cfg(not(feature = "hybrid"))]
    (),
);
#[derive(Debug)]
pub(crate) struct ReverseHybridEngine(
    #[cfg(feature = "hybrid")]
    hybrid::dfa::DFA,
    #[cfg(not(feature = "hybrid"))]
    (),
);
impl ReverseHybrid {
    pub(crate) fn none() -> ReverseHybrid {}
    pub(crate) fn new(info: &RegexInfo, nfarev: &NFA) -> ReverseHybrid {}
    pub(crate) fn create_cache(&self) -> ReverseHybridCache {
        ReverseHybridCache::new(self)
    }
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn get(&self, _input: &Input<'_>) -> Option<&ReverseHybridEngine> {}
}
impl ReverseHybridCache {
    pub(crate) fn none() -> ReverseHybridCache {}
    pub(crate) fn new(builder: &ReverseHybrid) -> ReverseHybridCache {
        #[cfg(feature = "hybrid")]
        { ReverseHybridCache(builder.0.as_ref().map(|e| e.0.create_cache())) }
        #[cfg(not(feature = "hybrid"))] { ReverseHybridCache(()) }
    }
    pub(crate) fn reset(&mut self, builder: &ReverseHybrid) {}
    pub(crate) fn memory_usage(&self) -> usize {}
}
