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
#[derive(Debug)]
pub(crate) struct BoundedBacktracker(Option<BoundedBacktrackerEngine>);
#[derive(Clone, Debug)]
pub struct BoundedBacktracker {
    config: Config,
    nfa: NFA,
}
#[derive(Clone)]
pub struct Input<'h> {
    haystack: &'h [u8],
    span: Span,
    anchored: Anchored,
    earliest: bool,
}
#[derive(Clone, Debug)]
pub struct Cache {
    /// Stack used on the heap for doing backtracking instead of the
    /// traditional recursive approach. We don't want recursion because then
    /// we're likely to hit a stack overflow for bigger regexes.
    stack: Vec<Frame>,
    /// The set of (StateID, HaystackOffset) pairs that have been visited
    /// by the backtracker within a single search. If such a pair has been
    /// visited, then we avoid doing the work for that pair again. This is
    /// what "bounds" the backtracking and prevents it from having worst case
    /// exponential time.
    visited: Visited,
}
#[derive(Clone, Debug)]
pub(crate) struct BoundedBacktrackerCache(
    #[cfg(feature = "nfa-backtrack")]
    Option<backtrack::Cache>,
    #[cfg(not(feature = "nfa-backtrack"))]
    (),
);
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchError(
    #[cfg(feature = "alloc")]
    alloc::boxed::Box<MatchErrorKind>,
    #[cfg(not(feature = "alloc"))]
    MatchErrorKind,
);
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
    ) -> bool {
        #[cfg(feature = "nfa-backtrack")]
        { self.0.try_is_match(cache.0.as_mut().unwrap(), input.clone()).unwrap() }
        #[cfg(not(feature = "nfa-backtrack"))] { unreachable!() }
    }
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
