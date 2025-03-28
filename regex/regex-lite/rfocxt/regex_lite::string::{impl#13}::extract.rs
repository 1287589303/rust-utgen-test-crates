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
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Match<'h> {
    haystack: &'h str,
    start: usize,
    end: usize,
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
#[derive(Clone, Debug)]
pub struct SubCaptureMatches<'c, 'h> {
    caps: &'c Captures<'h>,
    it: core::iter::Enumerate<nfa::CaptureNames<'c>>,
}
impl<'h> Captures<'h> {
    #[inline]
    pub fn get(&self, i: usize) -> Option<Match<'h>> {}
    #[inline]
    pub fn name(&self, name: &str) -> Option<Match<'h>> {}
    pub fn extract<const N: usize>(&self) -> (&'h str, [&'h str; N]) {
        let len = self
            .pikevm
            .nfa()
            .static_explicit_captures_len()
            .expect("number of capture groups can vary in a match");
        assert_eq!(N, len, "asked for {} groups, but must ask for {}", N, len);
        let mut matched = self.iter().flatten();
        let whole_match = matched.next().expect("a match").as_str();
        let group_matches = [0; N]
            .map(|_| { matched.next().expect("too few matching groups").as_str() });
        (whole_match, group_matches)
    }
    #[inline]
    pub fn expand(&self, replacement: &str, dst: &mut String) {}
    #[inline]
    pub fn iter<'c>(&'c self) -> SubCaptureMatches<'c, 'h> {
        SubCaptureMatches {
            caps: self,
            it: self.pikevm.nfa().capture_names().enumerate(),
        }
    }
    #[inline]
    pub fn len(&self) -> usize {}
}
impl<'h> Match<'h> {
    #[inline]
    fn new(haystack: &'h str, start: usize, end: usize) -> Match<'h> {}
    #[inline]
    pub fn start(&self) -> usize {}
    #[inline]
    pub fn end(&self) -> usize {}
    #[inline]
    pub fn is_empty(&self) -> bool {}
    #[inline]
    pub fn len(&self) -> usize {}
    #[inline]
    pub fn range(&self) -> core::ops::Range<usize> {}
    #[inline]
    pub fn as_str(&self) -> &'h str {
        &self.haystack[self.range()]
    }
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
    pub(crate) fn to_index(&self, name: &str) -> Option<usize> {}
    pub(crate) fn capture_names(&self) -> CaptureNames<'_> {}
    pub(crate) fn group_len(&self) -> usize {}
    pub(crate) fn is_start_anchored(&self) -> bool {}
    pub(crate) fn static_explicit_captures_len(&self) -> Option<usize> {
        self.static_explicit_captures_len
    }
    fn memory_usage(&self) -> usize {}
}
