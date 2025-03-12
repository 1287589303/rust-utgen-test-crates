use core::{fmt::Debug, panic::{RefUnwindSafe, UnwindSafe}};
use alloc::sync::Arc;
use regex_syntax::hir::{literal, Hir};
use crate::{
    meta::{
        error::{BuildError, RetryError, RetryFailError, RetryQuadraticError},
        regex::{Cache, RegexInfo},
        reverse_inner, wrappers,
    },
    nfa::thompson::{self, WhichCaptures, NFA},
    util::{
        captures::{Captures, GroupInfo},
        look::LookMatcher, prefilter::{self, Prefilter, PrefilterI},
        primitives::{NonMaxUsize, PatternID},
        search::{Anchored, HalfMatch, Input, Match, MatchKind, PatternSet},
    },
};
pub(super) trait Strategy: Debug + Send + Sync + RefUnwindSafe + UnwindSafe + 'static {
    fn group_info(&self) -> &GroupInfo;
    fn create_cache(&self) -> Cache;
    fn reset_cache(&self, cache: &mut Cache);
    fn is_accelerated(&self) -> bool;
    fn memory_usage(&self) -> usize;
    fn search(&self, cache: &mut Cache, input: &Input<'_>) -> Option<Match>;
    fn search_half(&self, cache: &mut Cache, input: &Input<'_>) -> Option<HalfMatch>;
    fn is_match(&self, cache: &mut Cache, input: &Input<'_>) -> bool;
    fn search_slots(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Option<PatternID>;
    fn which_overlapping_matches(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        patset: &mut PatternSet,
    );
}
pub(crate) trait PrefilterI: Debug + Send + Sync + RefUnwindSafe + UnwindSafe + 'static {
    fn find(&self, haystack: &[u8], span: Span) -> Option<Span>;
    fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span>;
    fn memory_usage(&self) -> usize;
    fn is_fast(&self) -> bool;
}
#[derive(Clone, Debug)]
struct Pre<P> {
    pre: P,
    group_info: GroupInfo,
}
#[derive(Clone, Debug)]
pub(crate) struct PikeVMCache(Option<pikevm::Cache>);
#[derive(Clone, Debug)]
pub(crate) struct HybridCache(
    #[cfg(feature = "hybrid")]
    Option<hybrid::regex::Cache>,
    #[cfg(not(feature = "hybrid"))]
    (),
);
#[derive(Clone, Debug)]
pub(crate) struct ReverseHybridCache(
    #[cfg(feature = "hybrid")]
    Option<hybrid::dfa::Cache>,
    #[cfg(not(feature = "hybrid"))]
    (),
);
#[derive(Clone)]
pub struct Captures {
    /// The group info that these capture groups are coupled to. This is what
    /// gives the "convenience" of the `Captures` API. Namely, it provides the
    /// slot mapping and the name|-->index mapping for capture lookups by name.
    group_info: GroupInfo,
    /// The ID of the pattern that matched. Regex engines must set this to
    /// None when no match occurs.
    pid: Option<PatternID>,
    /// The slot values, i.e., submatch offsets.
    ///
    /// In theory, the smallest sequence of slots would be something like
    /// `max(groups(pattern) for pattern in regex) * 2`, but instead, we use
    /// `sum(groups(pattern) for pattern in regex) * 2`. Why?
    ///
    /// Well, the former could be used in theory, because we don't generally
    /// have any overlapping APIs that involve capturing groups. Therefore,
    /// there's technically never any need to have slots set for multiple
    /// patterns. However, this might change some day, in which case, we would
    /// need to have slots available.
    ///
    /// The other reason is that during the execution of some regex engines,
    /// there exists a point in time where multiple slots for different
    /// patterns may be written to before knowing which pattern has matched.
    /// Therefore, the regex engines themselves, in order to support multiple
    /// patterns correctly, must have all slots available. If `Captures`
    /// doesn't have all slots available, then regex engines can't write
    /// directly into the caller provided `Captures` and must instead write
    /// into some other storage and then copy the slots involved in the match
    /// at the end of the search.
    ///
    /// So overall, at least as of the time of writing, it seems like the path
    /// of least resistance is to just require allocating all possible slots
    /// instead of the conceptual minimum. Another way to justify this is that
    /// the most common case is a single pattern, in which case, there is no
    /// inefficiency here since the 'max' and 'sum' calculations above are
    /// equivalent in that case.
    ///
    /// N.B. The mapping from group index to slot is maintained by `GroupInfo`
    /// and is considered an API guarantee. See `GroupInfo` for more details on
    /// that mapping.
    ///
    /// N.B. `Option<NonMaxUsize>` has the same size as a `usize`.
    slots: Vec<Option<NonMaxUsize>>,
}
#[derive(Clone, Debug)]
pub(crate) struct BoundedBacktrackerCache(
    #[cfg(feature = "nfa-backtrack")]
    Option<backtrack::Cache>,
    #[cfg(not(feature = "nfa-backtrack"))]
    (),
);
#[derive(Clone, Debug)]
pub(crate) struct OnePassCache(
    #[cfg(feature = "dfa-onepass")]
    Option<onepass::Cache>,
    #[cfg(not(feature = "dfa-onepass"))]
    (),
);
#[derive(Clone, Debug, Default)]
pub struct GroupInfo(Arc<GroupInfoInner>);
#[derive(Debug, Clone)]
pub struct Cache {
    pub(crate) capmatches: Captures,
    pub(crate) pikevm: wrappers::PikeVMCache,
    pub(crate) backtrack: wrappers::BoundedBacktrackerCache,
    pub(crate) onepass: wrappers::OnePassCache,
    pub(crate) hybrid: wrappers::HybridCache,
    pub(crate) revhybrid: wrappers::ReverseHybridCache,
}
impl<P: PrefilterI> Strategy for Pre<P> {
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn group_info(&self) -> &GroupInfo {}
    fn create_cache(&self) -> Cache {
        Cache {
            capmatches: Captures::all(self.group_info().clone()),
            pikevm: wrappers::PikeVMCache::none(),
            backtrack: wrappers::BoundedBacktrackerCache::none(),
            onepass: wrappers::OnePassCache::none(),
            hybrid: wrappers::HybridCache::none(),
            revhybrid: wrappers::ReverseHybridCache::none(),
        }
    }
    fn reset_cache(&self, _cache: &mut Cache) {}
    fn is_accelerated(&self) -> bool {}
    fn memory_usage(&self) -> usize {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn search(&self, _cache: &mut Cache, input: &Input<'_>) -> Option<Match> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn search_half(&self, cache: &mut Cache, input: &Input<'_>) -> Option<HalfMatch> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn is_match(&self, cache: &mut Cache, input: &Input<'_>) -> bool {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn search_slots(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Option<PatternID> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn which_overlapping_matches(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        patset: &mut PatternSet,
    ) {}
}
impl PikeVMCache {
    pub(crate) fn none() -> PikeVMCache {
        PikeVMCache(None)
    }
    pub(crate) fn new(builder: &PikeVM) -> PikeVMCache {}
    pub(crate) fn reset(&mut self, builder: &PikeVM) {}
    pub(crate) fn memory_usage(&self) -> usize {}
}
impl HybridCache {
    pub(crate) fn none() -> HybridCache {
        #[cfg(feature = "hybrid")] { HybridCache(None) }
        #[cfg(not(feature = "hybrid"))] { HybridCache(()) }
    }
    pub(crate) fn new(builder: &Hybrid) -> HybridCache {}
    pub(crate) fn reset(&mut self, builder: &Hybrid) {}
    pub(crate) fn memory_usage(&self) -> usize {}
}
impl ReverseHybridCache {
    pub(crate) fn none() -> ReverseHybridCache {
        #[cfg(feature = "hybrid")] { ReverseHybridCache(None) }
        #[cfg(not(feature = "hybrid"))] { ReverseHybridCache(()) }
    }
    pub(crate) fn new(builder: &ReverseHybrid) -> ReverseHybridCache {}
    pub(crate) fn reset(&mut self, builder: &ReverseHybrid) {}
    pub(crate) fn memory_usage(&self) -> usize {}
}
impl Captures {
    pub fn all(group_info: GroupInfo) -> Captures {
        let slots = group_info.slot_len();
        Captures {
            group_info,
            pid: None,
            slots: vec![None; slots],
        }
    }
    pub fn matches(group_info: GroupInfo) -> Captures {}
    pub fn empty(group_info: GroupInfo) -> Captures {}
    #[inline]
    pub fn is_match(&self) -> bool {}
    #[inline]
    pub fn pattern(&self) -> Option<PatternID> {}
    #[inline]
    pub fn get_match(&self) -> Option<Match> {}
    #[inline]
    pub fn get_group(&self, index: usize) -> Option<Span> {}
    pub fn get_group_by_name(&self, name: &str) -> Option<Span> {}
    pub fn iter(&self) -> CapturesPatternIter<'_> {}
    pub fn group_len(&self) -> usize {}
    pub fn group_info(&self) -> &GroupInfo {}
    pub fn interpolate_string(&self, haystack: &str, replacement: &str) -> String {}
    pub fn interpolate_string_into(
        &self,
        haystack: &str,
        replacement: &str,
        dst: &mut String,
    ) {}
    pub fn interpolate_bytes(&self, haystack: &[u8], replacement: &[u8]) -> Vec<u8> {}
    pub fn interpolate_bytes_into(
        &self,
        haystack: &[u8],
        replacement: &[u8],
        dst: &mut Vec<u8>,
    ) {}
    pub fn extract<'h, const N: usize>(
        &self,
        haystack: &'h str,
    ) -> (&'h str, [&'h str; N]) {}
    pub fn extract_bytes<'h, const N: usize>(
        &self,
        haystack: &'h [u8],
    ) -> (&'h [u8], [&'h [u8]; N]) {}
}
impl BoundedBacktrackerCache {
    pub(crate) fn none() -> BoundedBacktrackerCache {
        #[cfg(feature = "nfa-backtrack")] { BoundedBacktrackerCache(None) }
        #[cfg(not(feature = "nfa-backtrack"))] { BoundedBacktrackerCache(()) }
    }
    pub(crate) fn new(builder: &BoundedBacktracker) -> BoundedBacktrackerCache {}
    pub(crate) fn reset(&mut self, builder: &BoundedBacktracker) {}
    pub(crate) fn memory_usage(&self) -> usize {}
}
impl OnePassCache {
    pub(crate) fn none() -> OnePassCache {
        #[cfg(feature = "dfa-onepass")] { OnePassCache(None) }
        #[cfg(not(feature = "dfa-onepass"))] { OnePassCache(()) }
    }
    pub(crate) fn new(builder: &OnePass) -> OnePassCache {}
    pub(crate) fn reset(&mut self, builder: &OnePass) {}
    pub(crate) fn memory_usage(&self) -> usize {}
}
