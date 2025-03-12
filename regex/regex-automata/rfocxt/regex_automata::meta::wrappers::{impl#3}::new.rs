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
#[derive(Debug)]
pub(crate) struct BoundedBacktrackerEngine(
    #[cfg(feature = "nfa-backtrack")]
    backtrack::BoundedBacktracker,
    #[cfg(not(feature = "nfa-backtrack"))]
    (),
);
#[derive(Clone)]
pub struct NFA(Arc<Inner>);
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
pub struct BuildError {
    kind: BuildErrorKind,
}
#[derive(Clone, Debug)]
pub(crate) struct RegexInfo(Arc<RegexInfoI>);
impl BoundedBacktracker {
    pub(crate) fn new(
        info: &RegexInfo,
        pre: Option<Prefilter>,
        nfa: &NFA,
    ) -> Result<BoundedBacktracker, BuildError> {
        BoundedBacktrackerEngine::new(info, pre, nfa).map(BoundedBacktracker)
    }
    pub(crate) fn create_cache(&self) -> BoundedBacktrackerCache {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn get(&self, input: &Input<'_>) -> Option<&BoundedBacktrackerEngine> {}
}
impl BoundedBacktrackerEngine {
    pub(crate) fn new(
        info: &RegexInfo,
        pre: Option<Prefilter>,
        nfa: &NFA,
    ) -> Result<Option<BoundedBacktrackerEngine>, BuildError> {
        #[cfg(feature = "nfa-backtrack")]
        {
            if !info.config().get_backtrack()
                || info.config().get_match_kind() != MatchKind::LeftmostFirst
            {
                return Ok(None);
            }
            let backtrack_config = backtrack::Config::new().prefilter(pre);
            let engine = backtrack::Builder::new()
                .configure(backtrack_config)
                .build_from_nfa(nfa.clone())
                .map_err(BuildError::nfa)?;
            debug!(
                "BoundedBacktracker built (max haystack length: {:?})", engine
                .max_haystack_len()
            );
            Ok(Some(BoundedBacktrackerEngine(engine)))
        }
        #[cfg(not(feature = "nfa-backtrack"))] { Ok(None) }
    }
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
    fn max_haystack_len(&self) -> usize {}
}
