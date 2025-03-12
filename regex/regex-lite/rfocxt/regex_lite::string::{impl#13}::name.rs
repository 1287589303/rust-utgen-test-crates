use alloc::{
    borrow::Cow, boxed::Box, string::String, string::ToString, sync::Arc, vec, vec::Vec,
};
use crate::{
    error::Error, hir::{self, Hir},
    int::NonMaxUsize, interpolate, nfa::{self, NFA},
    pikevm::{self, Cache, PikeVM},
    pool::CachePool,
};
pub struct Captures<'h> {
    haystack: &'h str,
    slots: CaptureLocations,
    pikevm: Arc<PikeVM>,
}
#[derive(Clone, Debug)]
pub(crate) struct PikeVM {
    nfa: NFA,
}
#[derive(Clone)]
pub(crate) struct NFA {
    /// The pattern string this NFA was generated from.
    ///
    /// We put it here for lack of a better place to put it. ¯\_(ツ)_/¯
    pattern: String,
    /// The states that make up this NFA.
    states: Vec<State>,
    /// The ID of the start state.
    start: StateID,
    /// Whether this NFA can only match at the beginning of a haystack.
    is_start_anchored: bool,
    /// Whether this NFA can match the empty string.
    is_match_empty: bool,
    /// If every match has the same number of matching capture groups, then
    /// this corresponds to the number of groups.
    static_explicit_captures_len: Option<usize>,
    /// A map from capture group name to its corresponding index.
    cap_name_to_index: CaptureNameMap,
    /// A map from capture group index to the corresponding name, if one
    /// exists.
    cap_index_to_name: Vec<Option<Arc<str>>>,
    /// Heap memory used indirectly by NFA states and other things (like the
    /// various capturing group representations above). Since each state
    /// might use a different amount of heap, we need to keep track of this
    /// incrementally.
    memory_extra: usize,
}
#[derive(Clone, Debug)]
pub struct CaptureLocations(Vec<Option<NonMaxUsize>>);
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match<'h> {
    haystack: &'h str,
    start: usize,
    end: usize,
}
impl<'h> Captures<'h> {
    #[inline]
    pub fn get(&self, i: usize) -> Option<Match<'h>> {
        self.slots.get(i).map(|(s, e)| Match::new(self.haystack, s, e))
    }
    #[inline]
    pub fn name(&self, name: &str) -> Option<Match<'h>> {
        let i = self.pikevm.nfa().to_index(name)?;
        self.get(i)
    }
    pub fn extract<const N: usize>(&self) -> (&'h str, [&'h str; N]) {}
    #[inline]
    pub fn expand(&self, replacement: &str, dst: &mut String) {}
    #[inline]
    pub fn iter<'c>(&'c self) -> SubCaptureMatches<'c, 'h> {}
    #[inline]
    pub fn len(&self) -> usize {}
}
impl PikeVM {
    pub(crate) fn new(nfa: NFA) -> PikeVM {}
    pub(crate) fn nfa(&self) -> &NFA {
        &self.nfa
    }
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
    ) -> bool {}
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
impl NFA {
    pub(crate) fn new(config: Config, pattern: String, hir: &Hir) -> Result<NFA, Error> {}
    pub(crate) fn pattern(&self) -> &str {}
    pub(crate) fn state(&self, id: StateID) -> &State {}
    pub(crate) fn len(&self) -> usize {}
    pub(crate) fn start(&self) -> StateID {}
    pub(crate) fn to_index(&self, name: &str) -> Option<usize> {
        self.cap_name_to_index.get(name).cloned().map(|i| i.as_usize())
    }
    pub(crate) fn capture_names(&self) -> CaptureNames<'_> {}
    pub(crate) fn group_len(&self) -> usize {}
    pub(crate) fn is_start_anchored(&self) -> bool {}
    pub(crate) fn static_explicit_captures_len(&self) -> Option<usize> {}
    fn memory_usage(&self) -> usize {}
}
