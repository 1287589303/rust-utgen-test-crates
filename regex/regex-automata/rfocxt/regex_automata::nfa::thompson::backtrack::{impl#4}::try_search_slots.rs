use alloc::{vec, vec::Vec};
use crate::{
    nfa::thompson::{self, BuildError, State, NFA},
    util::{
        captures::Captures, empty, iter, prefilter::Prefilter,
        primitives::{NonMaxUsize, PatternID, SmallIndex, StateID},
        search::{Anchored, HalfMatch, Input, Match, MatchError, Span},
    },
};
#[derive(Clone, Debug)]
pub struct BoundedBacktracker {
    config: Config,
    nfa: NFA,
}
#[derive(Clone)]
pub struct NFA(Arc<Inner>);
#[derive(Clone, Debug, Default)]
pub struct GroupInfo(Arc<GroupInfoInner>);
#[derive(Clone, Debug, Default)]
pub struct Config {
    match_kind: Option<MatchKind>,
    starts_for_each_pattern: Option<bool>,
    byte_classes: Option<bool>,
    size_limit: Option<Option<usize>>,
}
#[derive(Clone)]
pub struct Input<'h> {
    haystack: &'h [u8],
    span: Span,
    anchored: Anchored,
    earliest: bool,
}
#[derive(Clone, Debug)]
pub struct Config {
    look_behind: Option<u8>,
    anchored: Anchored,
}
#[derive(Clone, Debug, Default)]
pub struct Config {
    match_kind: Option<MatchKind>,
    pre: Option<Option<Prefilter>>,
    starts_for_each_pattern: Option<bool>,
    byte_classes: Option<bool>,
    unicode_word_boundary: Option<bool>,
    quitset: Option<ByteSet>,
    specialize_start_states: Option<bool>,
    cache_capacity: Option<usize>,
    skip_cache_capacity_check: Option<bool>,
    minimum_cache_clear_count: Option<Option<usize>>,
    minimum_bytes_per_state: Option<Option<usize>>,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct HalfMatch {
    /// The pattern ID.
    pattern: PatternID,
    /// The offset of the match.
    ///
    /// For forward searches, the offset is exclusive. For reverse searches,
    /// the offset is inclusive.
    offset: usize,
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchError(
    #[cfg(feature = "alloc")]
    alloc::boxed::Box<MatchErrorKind>,
    #[cfg(not(feature = "alloc"))]
    MatchErrorKind,
);
#[derive(Clone, Debug, Default)]
pub struct Config {
    match_kind: Option<MatchKind>,
    pre: Option<Option<Prefilter>>,
}
#[derive(Clone, Debug, Default)]
pub struct Config {
    utf8: Option<bool>,
    reverse: Option<bool>,
    nfa_size_limit: Option<Option<usize>>,
    shrink: Option<bool>,
    which_captures: Option<WhichCaptures>,
    look_matcher: Option<LookMatcher>,
    #[cfg(test)]
    unanchored_prefix: Option<bool>,
}
#[cfg(feature = "dfa-build")]
#[derive(Clone, Debug, Default)]
pub struct Config {
    accelerate: Option<bool>,
    pre: Option<Option<Prefilter>>,
    minimize: Option<bool>,
    match_kind: Option<MatchKind>,
    start_kind: Option<StartKind>,
    starts_for_each_pattern: Option<bool>,
    byte_classes: Option<bool>,
    unicode_word_boundary: Option<bool>,
    quitset: Option<ByteSet>,
    specialize_start_states: Option<bool>,
    dfa_size_limit: Option<Option<usize>>,
    determinize_size_limit: Option<Option<usize>>,
}
#[derive(Clone, Debug)]
pub(crate) struct Config {
    match_kind: MatchKind,
    quit: ByteSet,
    dfa_size_limit: Option<usize>,
    determinize_size_limit: Option<usize>,
}
#[derive(Clone, Copy, Debug)]
pub struct Config {
    case_insensitive: bool,
    multi_line: bool,
    dot_matches_new_line: bool,
    crlf: bool,
    line_terminator: u8,
    swap_greed: bool,
    ignore_whitespace: bool,
    unicode: bool,
    utf8: bool,
    nest_limit: u32,
    octal: bool,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
#[derive(Clone, Debug, Default)]
pub struct Config {
    match_kind: Option<MatchKind>,
    utf8_empty: Option<bool>,
    autopre: Option<bool>,
    pre: Option<Option<Prefilter>>,
    which_captures: Option<WhichCaptures>,
    nfa_size_limit: Option<Option<usize>>,
    onepass_size_limit: Option<Option<usize>>,
    hybrid_cache_capacity: Option<usize>,
    hybrid: Option<bool>,
    dfa: Option<bool>,
    dfa_size_limit: Option<Option<usize>>,
    dfa_state_limit: Option<Option<usize>>,
    onepass: Option<bool>,
    backtrack: Option<bool>,
    byte_classes: Option<bool>,
    line_terminator: Option<u8>,
}
#[derive(Clone, Debug, Default)]
pub struct Config {
    pre: Option<Option<Prefilter>>,
    visited_capacity: Option<usize>,
}
#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct NonMaxUsize(NonZeroUsize);
impl BoundedBacktracker {
    #[inline]
    pub fn try_search(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        caps: &mut Captures,
    ) -> Result<(), MatchError> {}
    #[inline]
    pub fn try_search_slots(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Result<Option<PatternID>, MatchError> {
        let utf8empty = self.get_nfa().has_empty() && self.get_nfa().is_utf8();
        if !utf8empty {
            let maybe_hm = self.try_search_slots_imp(cache, input, slots)?;
            return Ok(maybe_hm.map(|hm| hm.pattern()));
        }
        let min = self.get_nfa().group_info().implicit_slot_len();
        if slots.len() >= min {
            let maybe_hm = self.try_search_slots_imp(cache, input, slots)?;
            return Ok(maybe_hm.map(|hm| hm.pattern()));
        }
        if self.get_nfa().pattern_len() == 1 {
            let mut enough = [None, None];
            let got = self.try_search_slots_imp(cache, input, &mut enough)?;
            slots.copy_from_slice(&enough[..slots.len()]);
            return Ok(got.map(|hm| hm.pattern()));
        }
        let mut enough = vec![None; min];
        let got = self.try_search_slots_imp(cache, input, &mut enough)?;
        slots.copy_from_slice(&enough[..slots.len()]);
        Ok(got.map(|hm| hm.pattern()))
    }
    #[inline(never)]
    fn try_search_slots_imp(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Result<Option<HalfMatch>, MatchError> {
        let utf8empty = self.get_nfa().has_empty() && self.get_nfa().is_utf8();
        let hm = match self.search_imp(cache, input, slots)? {
            None => return Ok(None),
            Some(hm) if !utf8empty => return Ok(Some(hm)),
            Some(hm) => hm,
        };
        empty::skip_splits_fwd(
            input,
            hm,
            hm.offset(),
            |input| {
                Ok(self.search_imp(cache, input, slots)?.map(|hm| (hm, hm.offset())))
            },
        )
    }
    fn search_imp(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Result<Option<HalfMatch>, MatchError> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn backtrack(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        at: usize,
        start_id: StateID,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Option<HalfMatch> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn step(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        mut sid: StateID,
        mut at: usize,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Option<HalfMatch> {}
}
impl NFA {
    #[cfg(feature = "syntax")]
    pub fn new(pattern: &str) -> Result<NFA, BuildError> {}
    #[cfg(feature = "syntax")]
    pub fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<NFA, BuildError> {}
    pub fn always_match() -> NFA {}
    pub fn never_match() -> NFA {}
    #[cfg(feature = "syntax")]
    pub fn config() -> Config {}
    #[cfg(feature = "syntax")]
    pub fn compiler() -> Compiler {}
    pub fn patterns(&self) -> PatternIter<'_> {}
    #[inline]
    pub fn pattern_len(&self) -> usize {
        self.0.start_pattern.len()
    }
    #[inline]
    pub fn start_anchored(&self) -> StateID {}
    #[inline]
    pub fn start_unanchored(&self) -> StateID {}
    #[inline]
    pub fn start_pattern(&self, pid: PatternID) -> Option<StateID> {}
    #[inline]
    pub(crate) fn byte_class_set(&self) -> &ByteClassSet {}
    #[inline]
    pub fn byte_classes(&self) -> &ByteClasses {}
    #[inline]
    pub fn state(&self, id: StateID) -> &State {}
    #[inline]
    pub fn states(&self) -> &[State] {}
    #[inline]
    pub fn group_info(&self) -> &GroupInfo {
        &self.0.group_info()
    }
    #[inline]
    pub fn has_capture(&self) -> bool {}
    #[inline]
    pub fn has_empty(&self) -> bool {
        self.0.has_empty
    }
    #[inline]
    pub fn is_utf8(&self) -> bool {
        self.0.utf8
    }
    #[inline]
    pub fn is_reverse(&self) -> bool {}
    #[inline]
    pub fn is_always_start_anchored(&self) -> bool {}
    #[inline]
    pub fn look_matcher(&self) -> &LookMatcher {}
    #[inline]
    pub fn look_set_any(&self) -> LookSet {}
    #[inline]
    pub fn look_set_prefix_any(&self) -> LookSet {}
    #[inline]
    pub fn memory_usage(&self) -> usize {}
}
impl GroupInfo {
    pub fn new<P, G, N>(pattern_groups: P) -> Result<GroupInfo, GroupInfoError>
    where
        P: IntoIterator<Item = G>,
        G: IntoIterator<Item = Option<N>>,
        N: AsRef<str>,
    {}
    pub fn empty() -> GroupInfo {}
    #[inline]
    pub fn to_index(&self, pid: PatternID, name: &str) -> Option<usize> {}
    #[inline]
    pub fn to_name(&self, pid: PatternID, group_index: usize) -> Option<&str> {}
    #[inline]
    pub fn pattern_names(&self, pid: PatternID) -> GroupInfoPatternNames<'_> {}
    #[inline]
    pub fn all_names(&self) -> GroupInfoAllNames<'_> {}
    #[inline]
    pub fn slots(&self, pid: PatternID, group_index: usize) -> Option<(usize, usize)> {}
    #[inline]
    pub fn slot(&self, pid: PatternID, group_index: usize) -> Option<usize> {}
    #[inline]
    pub fn pattern_len(&self) -> usize {}
    #[inline]
    pub fn group_len(&self, pid: PatternID) -> usize {}
    #[inline]
    pub fn all_group_len(&self) -> usize {}
    #[inline]
    pub fn slot_len(&self) -> usize {}
    #[inline]
    pub fn implicit_slot_len(&self) -> usize {
        self.pattern_len() * 2
    }
    #[inline]
    pub fn explicit_slot_len(&self) -> usize {}
    #[inline]
    pub fn memory_usage(&self) -> usize {}
}
