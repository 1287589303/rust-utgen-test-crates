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
#[derive(Debug)]
pub struct Regex {
    /// The actual regex implementation.
    imp: Arc<RegexI>,
    /// A thread safe pool of caches.
    ///
    /// For the higher level search APIs, a `Cache` is automatically plucked
    /// from this pool before running a search. The lower level `with` methods
    /// permit the caller to provide their own cache, thereby bypassing
    /// accesses to this pool.
    ///
    /// Note that we put this outside the `Arc` so that cloning a `Regex`
    /// results in creating a fresh `CachePool`. This in turn permits callers
    /// to clone regexes into separate threads where each such regex gets
    /// the pool's "thread owner" optimization. Otherwise, if one shares the
    /// `Regex` directly, then the pool will go through a slower mutex path for
    /// all threads except for the "owner."
    pool: CachePool,
}
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
#[derive(Clone)]
pub struct Input<'h> {
    haystack: &'h [u8],
    span: Span,
    anchored: Anchored,
    earliest: bool,
}
#[derive(Debug)]
struct RegexI {
    /// The core matching engine.
    ///
    /// Why is this reference counted when RegexI is already wrapped in an Arc?
    /// Well, we need to capture this in a closure to our `Pool` below in order
    /// to create new `Cache` values when needed. So since it needs to be in
    /// two places, we make it reference counted.
    ///
    /// We make `RegexI` itself reference counted too so that `Regex` itself
    /// stays extremely small and very cheap to clone.
    strat: Arc<dyn Strategy>,
    /// Metadata about the regexes driving the strategy. The metadata is also
    /// usually stored inside the strategy too, but we put it here as well
    /// so that we can get quick access to it (without virtual calls) before
    /// executing the regex engine. For example, we use this metadata to
    /// detect a subset of cases where we know a match is impossible, and can
    /// thus avoid calling into the strategy at all.
    ///
    /// Since `RegexInfo` is stored in multiple places, it is also reference
    /// counted.
    info: RegexInfo,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct NonMaxUsize(NonZeroUsize);
impl Regex {
    #[inline]
    pub fn search(&self, input: &Input<'_>) -> Option<Match> {}
    #[inline]
    pub fn search_half(&self, input: &Input<'_>) -> Option<HalfMatch> {}
    #[inline]
    pub fn search_captures(&self, input: &Input<'_>, caps: &mut Captures) {
        caps.set_pattern(None);
        let pid = self.search_slots(input, caps.slots_mut());
        caps.set_pattern(pid);
    }
    #[inline]
    pub fn search_slots(
        &self,
        input: &Input<'_>,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Option<PatternID> {
        if self.imp.info.is_impossible(input) {
            return None;
        }
        let mut guard = self.pool.get();
        let result = self.imp.strat.search_slots(&mut guard, input, slots);
        PoolGuard::put(guard);
        result
    }
    #[inline]
    pub fn which_overlapping_matches(&self, input: &Input<'_>, patset: &mut PatternSet) {}
}
impl Captures {
    #[inline]
    pub fn clear(&mut self) {}
    #[inline]
    pub fn set_pattern(&mut self, pid: Option<PatternID>) {
        self.pid = pid;
    }
    #[inline]
    pub fn slots(&self) -> &[Option<NonMaxUsize>] {}
    #[inline]
    pub fn slots_mut(&mut self) -> &mut [Option<NonMaxUsize>] {
        &mut self.slots
    }
}
