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
pub struct PikeVM {
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
#[derive(Clone, Debug)]
pub(crate) struct PikeVMCache(Option<pikevm::Cache>);
#[derive(Debug)]
pub(crate) struct PikeVM(PikeVMEngine);
#[cfg(feature = "alloc")]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PatternSet {
    /// The number of patterns set to 'true' in this set.
    len: usize,
    /// A map from PatternID to boolean of whether a pattern matches or not.
    ///
    /// This should probably be a bitset, but it's probably unlikely to matter
    /// much in practice.
    ///
    /// The main downside of this representation (and similarly for a bitset)
    /// is that iteration scales with the capacity of the set instead of
    /// the length of the set. This doesn't seem likely to be a problem in
    /// practice.
    ///
    /// Another alternative is to just use a 'SparseSet' for this. It does use
    /// more memory (quite a bit more), but that seems fine I think compared
    /// to the memory being used by the regex engine. The real hiccup with
    /// it is that it yields pattern IDs in the order they were inserted.
    /// Which is actually kind of nice, but at the time of writing, pattern
    /// IDs are yielded in ascending order in the regex crate RegexSet API.
    /// If we did change to 'SparseSet', we could provide an additional
    /// 'iter_match_order' iterator, but keep the ascending order one for
    /// compatibility.
    which: alloc::boxed::Box<[bool]>,
}
impl PikeVMEngine {
    pub(crate) fn new(
        info: &RegexInfo,
        pre: Option<Prefilter>,
        nfa: &NFA,
    ) -> Result<PikeVMEngine, BuildError> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn is_match(&self, cache: &mut PikeVMCache, input: &Input<'_>) -> bool {}
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
    ) {
        self.0.which_overlapping_matches(cache.0.as_mut().unwrap(), input, patset)
    }
}
impl PikeVM {
    #[inline]
    pub fn search(&self, cache: &mut Cache, input: &Input<'_>, caps: &mut Captures) {}
    #[inline]
    pub fn search_slots(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Option<PatternID> {}
    #[inline(never)]
    fn search_slots_imp(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Option<HalfMatch> {}
    #[inline]
    pub fn which_overlapping_matches(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        patset: &mut PatternSet,
    ) {
        self.which_overlapping_imp(cache, input, patset)
    }
}
