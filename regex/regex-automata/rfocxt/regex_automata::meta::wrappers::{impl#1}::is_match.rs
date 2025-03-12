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
pub(crate) struct PikeVMEngine(pikevm::PikeVM);
#[derive(Clone, Debug)]
pub struct Cache {
    /// Stack used while computing epsilon closure. This effectively lets us
    /// move what is more naturally expressed through recursion to a stack
    /// on the heap.
    stack: Vec<FollowEpsilon>,
    /// The current active states being explored for the current byte in the
    /// haystack.
    curr: ActiveStates,
    /// The next set of states we're building that will be explored for the
    /// next byte in the haystack.
    next: ActiveStates,
}
#[derive(Clone)]
pub struct Input<'h> {
    haystack: &'h [u8],
    span: Span,
    anchored: Anchored,
    earliest: bool,
}
#[derive(Clone, Debug)]
pub(crate) struct PikeVMCache(Option<pikevm::Cache>);
#[derive(Clone, Debug)]
pub struct PikeVM {
    config: Config,
    nfa: NFA,
}
#[derive(Debug)]
pub(crate) struct PikeVM(PikeVMEngine);
impl PikeVMEngine {
    pub(crate) fn new(
        info: &RegexInfo,
        pre: Option<Prefilter>,
        nfa: &NFA,
    ) -> Result<PikeVMEngine, BuildError> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn is_match(&self, cache: &mut PikeVMCache, input: &Input<'_>) -> bool {
        self.0.is_match(cache.0.as_mut().unwrap(), input.clone())
    }
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
    ) {}
}
