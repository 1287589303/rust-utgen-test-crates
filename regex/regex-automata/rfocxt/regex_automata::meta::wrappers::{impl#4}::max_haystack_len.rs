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
pub(crate) struct BoundedBacktrackerEngine(
    #[cfg(feature = "nfa-backtrack")]
    backtrack::BoundedBacktracker,
    #[cfg(not(feature = "nfa-backtrack"))]
    (),
);
#[derive(Clone, Debug)]
pub struct BoundedBacktracker {
    config: Config,
    nfa: NFA,
}
#[derive(Debug)]
pub(crate) struct BoundedBacktracker(Option<BoundedBacktrackerEngine>);
impl BoundedBacktrackerEngine {
    pub(crate) fn new(
        info: &RegexInfo,
        pre: Option<Prefilter>,
        nfa: &NFA,
    ) -> Result<Option<BoundedBacktrackerEngine>, BuildError> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn is_match(
        &self,
        cache: &mut BoundedBacktrackerCache,
        input: &Input<'_>,
    ) -> bool {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn search_slots(
        &self,
        cache: &mut BoundedBacktrackerCache,
        input: &Input<'_>,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Option<PatternID> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn max_haystack_len(&self) -> usize {
        #[cfg(feature = "nfa-backtrack")] { self.0.max_haystack_len() }
        #[cfg(not(feature = "nfa-backtrack"))] { unreachable!() }
    }
}
impl BoundedBacktracker {
    #[cfg(feature = "syntax")]
    pub fn new(pattern: &str) -> Result<BoundedBacktracker, BuildError> {}
    #[cfg(feature = "syntax")]
    pub fn new_many<P: AsRef<str>>(
        patterns: &[P],
    ) -> Result<BoundedBacktracker, BuildError> {}
    pub fn new_from_nfa(nfa: NFA) -> Result<BoundedBacktracker, BuildError> {}
    pub fn always_match() -> Result<BoundedBacktracker, BuildError> {}
    pub fn never_match() -> Result<BoundedBacktracker, BuildError> {}
    pub fn config() -> Config {}
    pub fn builder() -> Builder {}
    pub fn create_cache(&self) -> Cache {}
    pub fn create_captures(&self) -> Captures {}
    pub fn reset_cache(&self, cache: &mut Cache) {}
    pub fn pattern_len(&self) -> usize {}
    #[inline]
    pub fn get_config(&self) -> &Config {}
    #[inline]
    pub fn get_nfa(&self) -> &NFA {}
    #[inline]
    pub fn max_haystack_len(&self) -> usize {
        let capacity = 8 * self.get_config().get_visited_capacity();
        let blocks = div_ceil(capacity, Visited::BLOCK_SIZE);
        let real_capacity = blocks.saturating_mul(Visited::BLOCK_SIZE);
        (real_capacity / self.nfa.states().len()).saturating_sub(1)
    }
}
