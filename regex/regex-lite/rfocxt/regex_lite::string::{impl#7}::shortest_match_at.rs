use alloc::{
    borrow::Cow, boxed::Box, string::String, string::ToString, sync::Arc, vec, vec::Vec,
};
use crate::{
    error::Error, hir::{self, Hir},
    int::NonMaxUsize, interpolate, nfa::{self, NFA},
    pikevm::{self, Cache, PikeVM},
    pool::CachePool,
};
pub struct Regex {
    pikevm: Arc<PikeVM>,
    pool: CachePool,
}
#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub(crate) struct NonMaxUsize(NonZeroUsize);
#[derive(Clone, Debug)]
pub(crate) struct PikeVM {
    nfa: NFA,
}
#[derive(Clone, Debug)]
pub(crate) struct Cache {
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
impl Regex {
    #[inline]
    pub fn shortest_match(&self, haystack: &str) -> Option<usize> {}
    #[inline]
    pub fn shortest_match_at(&self, haystack: &str, start: usize) -> Option<usize> {
        let mut cache = self.pool.get();
        let mut slots = [None, None];
        let matched = self
            .pikevm
            .search(
                &mut cache,
                haystack.as_bytes(),
                start,
                haystack.len(),
                true,
                &mut slots,
            );
        if !matched {
            return None;
        }
        Some(slots[1].unwrap().get())
    }
    #[inline]
    pub fn is_match_at(&self, haystack: &str, start: usize) -> bool {}
    #[inline]
    pub fn find_at<'h>(&self, haystack: &'h str, start: usize) -> Option<Match<'h>> {}
    #[inline]
    pub fn captures_at<'h>(
        &self,
        haystack: &'h str,
        start: usize,
    ) -> Option<Captures<'h>> {}
    #[inline]
    pub fn captures_read<'h>(
        &self,
        locs: &mut CaptureLocations,
        haystack: &'h str,
    ) -> Option<Match<'h>> {}
    #[inline]
    pub fn captures_read_at<'h>(
        &self,
        locs: &mut CaptureLocations,
        haystack: &'h str,
        start: usize,
    ) -> Option<Match<'h>> {}
}
impl NonMaxUsize {
    pub(crate) fn new(value: usize) -> Option<NonMaxUsize> {}
    pub(crate) fn get(self) -> usize {
        self.0.get().wrapping_sub(1)
    }
}
impl PikeVM {
    pub(crate) fn new(nfa: NFA) -> PikeVM {}
    pub(crate) fn nfa(&self) -> &NFA {}
    pub(crate) fn find_iter<'r, 'h>(
        &'r self,
        cache: CachePoolGuard<'r>,
        haystack: &'h [u8],
    ) -> FindMatches<'r, 'h> {}
    pub(crate) fn captures_iter<'r, 'h>(
        &'r self,
        cache: CachePoolGuard<'r>,
        haystack: &'h [u8],
    ) -> CapturesMatches<'r, 'h> {}
    pub(crate) fn search(
        &self,
        cache: &mut Cache,
        haystack: &[u8],
        start: usize,
        end: usize,
        earliest: bool,
        slots: &mut [Option<NonMaxUsize>],
    ) -> bool {
        cache.setup_search(slots.len());
        if start > end {
            return false;
        }
        assert!(
            haystack.len() < core::usize::MAX,
            "byte slice lengths must be less than usize MAX",
        );
        let Cache { ref mut stack, ref mut curr, ref mut next } = cache;
        let start_id = self.nfa().start();
        let anchored = self.nfa().is_start_anchored();
        let mut matched = false;
        let mut at = start;
        while at <= end {
            if curr.set.is_empty() {
                if matched {
                    break;
                }
                if anchored && at > start {
                    break;
                }
            }
            if !matched {
                let slots = next.slot_table.all_absent();
                self.epsilon_closure(stack, slots, curr, haystack, at, start_id);
            }
            let (ch, len) = utf8::decode_lossy(&haystack[at..]);
            if self.nexts(stack, curr, next, haystack, at, ch, len, slots) {
                matched = true;
            }
            if (earliest && matched) || len == 0 {
                break;
            }
            core::mem::swap(curr, next);
            next.set.clear();
            at += len;
        }
        matched
    }
    fn nexts(
        &self,
        stack: &mut Vec<FollowEpsilon>,
        curr: &mut ActiveStates,
        next: &mut ActiveStates,
        haystack: &[u8],
        at: usize,
        at_ch: char,
        at_len: usize,
        slots: &mut [Option<NonMaxUsize>],
    ) -> bool {}
    fn next(
        &self,
        stack: &mut Vec<FollowEpsilon>,
        curr_slot_table: &mut SlotTable,
        next: &mut ActiveStates,
        haystack: &[u8],
        at: usize,
        at_ch: char,
        at_len: usize,
        sid: StateID,
    ) -> bool {}
    fn epsilon_closure(
        &self,
        stack: &mut Vec<FollowEpsilon>,
        curr_slots: &mut [Option<NonMaxUsize>],
        next: &mut ActiveStates,
        haystack: &[u8],
        at: usize,
        sid: StateID,
    ) {}
    fn epsilon_closure_explore(
        &self,
        stack: &mut Vec<FollowEpsilon>,
        curr_slots: &mut [Option<NonMaxUsize>],
        next: &mut ActiveStates,
        haystack: &[u8],
        at: usize,
        mut sid: StateID,
    ) {}
}
