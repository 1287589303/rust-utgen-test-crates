type CachePool = Pool<Cache, CachePoolFn>;
type CachePoolGuard<'a> = PoolGuard<'a, Cache, CachePoolFn>;
type CachePoolFn = Box<dyn Fn() -> Cache + Send + Sync + UnwindSafe + RefUnwindSafe>;
use core::{borrow::Borrow, panic::{RefUnwindSafe, UnwindSafe}};
use alloc::{boxed::Box, sync::Arc, vec, vec::Vec};
use regex_syntax::{ast, hir::{self, Hir}};
use crate::{
    meta::{
        error::BuildError, strategy::{self, Strategy},
        wrappers,
    },
    nfa::thompson::WhichCaptures,
    util::{
        captures::{Captures, GroupInfo},
        iter, pool::{Pool, PoolGuard},
        prefilter::Prefilter, primitives::{NonMaxUsize, PatternID},
        search::{HalfMatch, Input, Match, MatchKind, PatternSet, Span},
    },
};
#[derive(Debug, Clone)]
pub struct Cache {
    pub(crate) capmatches: Captures,
    pub(crate) pikevm: wrappers::PikeVMCache,
    pub(crate) backtrack: wrappers::BoundedBacktrackerCache,
    pub(crate) onepass: wrappers::OnePassCache,
    pub(crate) hybrid: wrappers::HybridCache,
    pub(crate) revhybrid: wrappers::ReverseHybridCache,
}
#[derive(Clone, Debug)]
pub(crate) struct PikeVMCache(Option<pikevm::Cache>);
#[derive(Clone, Debug)]
pub(crate) struct ReverseHybridCache(
    #[cfg(feature = "hybrid")]
    Option<hybrid::dfa::Cache>,
    #[cfg(not(feature = "hybrid"))]
    (),
);
#[derive(Clone, Debug)]
pub(crate) struct HybridCache(
    #[cfg(feature = "hybrid")]
    Option<hybrid::regex::Cache>,
    #[cfg(not(feature = "hybrid"))]
    (),
);
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
impl Cache {
    pub fn new(re: &Regex) -> Cache {}
    pub fn reset(&mut self, re: &Regex) {}
    pub fn memory_usage(&self) -> usize {
        let mut bytes = 0;
        bytes += self.pikevm.memory_usage();
        bytes += self.backtrack.memory_usage();
        bytes += self.onepass.memory_usage();
        bytes += self.hybrid.memory_usage();
        bytes += self.revhybrid.memory_usage();
        bytes
    }
}
impl PikeVMCache {
    pub(crate) fn none() -> PikeVMCache {}
    pub(crate) fn new(builder: &PikeVM) -> PikeVMCache {}
    pub(crate) fn reset(&mut self, builder: &PikeVM) {}
    pub(crate) fn memory_usage(&self) -> usize {
        self.0.as_ref().map_or(0, |c| c.memory_usage())
    }
}
impl ReverseHybridCache {
    pub(crate) fn none() -> ReverseHybridCache {}
    pub(crate) fn new(builder: &ReverseHybrid) -> ReverseHybridCache {}
    pub(crate) fn reset(&mut self, builder: &ReverseHybrid) {}
    pub(crate) fn memory_usage(&self) -> usize {
        #[cfg(feature = "hybrid")] { self.0.as_ref().map_or(0, |c| c.memory_usage()) }
        #[cfg(not(feature = "hybrid"))] { 0 }
    }
}
impl HybridCache {
    pub(crate) fn none() -> HybridCache {}
    pub(crate) fn new(builder: &Hybrid) -> HybridCache {}
    pub(crate) fn reset(&mut self, builder: &Hybrid) {}
    pub(crate) fn memory_usage(&self) -> usize {
        #[cfg(feature = "hybrid")] { self.0.as_ref().map_or(0, |c| c.memory_usage()) }
        #[cfg(not(feature = "hybrid"))] { 0 }
    }
}
impl BoundedBacktrackerCache {
    pub(crate) fn none() -> BoundedBacktrackerCache {}
    pub(crate) fn new(builder: &BoundedBacktracker) -> BoundedBacktrackerCache {}
    pub(crate) fn reset(&mut self, builder: &BoundedBacktracker) {}
    pub(crate) fn memory_usage(&self) -> usize {
        #[cfg(feature = "nfa-backtrack")]
        { self.0.as_ref().map_or(0, |c| c.memory_usage()) }
        #[cfg(not(feature = "nfa-backtrack"))] { 0 }
    }
}
impl OnePassCache {
    pub(crate) fn none() -> OnePassCache {}
    pub(crate) fn new(builder: &OnePass) -> OnePassCache {}
    pub(crate) fn reset(&mut self, builder: &OnePass) {}
    pub(crate) fn memory_usage(&self) -> usize {
        #[cfg(feature = "dfa-onepass")]
        { self.0.as_ref().map_or(0, |c| c.memory_usage()) }
        #[cfg(not(feature = "dfa-onepass"))] { 0 }
    }
}
