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
pub(crate) struct BoundedBacktracker(Option<BoundedBacktrackerEngine>);
#[derive(Clone, Debug)]
pub(crate) struct BoundedBacktrackerCache(
    #[cfg(feature = "nfa-backtrack")]
    Option<backtrack::Cache>,
    #[cfg(not(feature = "nfa-backtrack"))]
    (),
);
#[derive(Debug)]
pub(crate) struct BoundedBacktrackerEngine(
    #[cfg(feature = "nfa-backtrack")]
    backtrack::BoundedBacktracker,
    #[cfg(not(feature = "nfa-backtrack"))]
    (),
);
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
impl BoundedBacktrackerCache {
    pub(crate) fn none() -> BoundedBacktrackerCache {}
    pub(crate) fn new(builder: &BoundedBacktracker) -> BoundedBacktrackerCache {
        #[cfg(feature = "nfa-backtrack")]
        { BoundedBacktrackerCache(builder.0.as_ref().map(|e| e.0.create_cache())) }
        #[cfg(not(feature = "nfa-backtrack"))] { BoundedBacktrackerCache(()) }
    }
    pub(crate) fn reset(&mut self, builder: &BoundedBacktracker) {}
    pub(crate) fn memory_usage(&self) -> usize {}
}
