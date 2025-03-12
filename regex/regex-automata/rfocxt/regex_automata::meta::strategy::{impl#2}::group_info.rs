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
#[derive(Clone, Debug, Default)]
pub struct GroupInfo(Arc<GroupInfoInner>);
impl<P: PrefilterI> Strategy for Pre<P> {
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn group_info(&self) -> &GroupInfo {
        &self.group_info
    }
    fn create_cache(&self) -> Cache {}
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
