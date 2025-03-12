#[cfg(feature = "internal-instrument-pikevm")]
use core::cell::RefCell;
use alloc::{vec, vec::Vec};
use crate::{
    nfa::thompson::{self, BuildError, State, NFA},
    util::{
        captures::Captures, empty, iter, prefilter::Prefilter,
        primitives::{NonMaxUsize, PatternID, SmallIndex, StateID},
        search::{Anchored, HalfMatch, Input, Match, MatchKind, PatternSet, Span},
        sparse_set::SparseSet,
    },
};
#[derive(Clone, Debug)]
pub struct PikeVM {
    config: Config,
    nfa: NFA,
}
#[derive(Clone)]
pub(crate) struct SparseSet {
    /// The number of elements currently in this set.
    len: usize,
    /// Dense contains the ids in the order in which they were inserted.
    dense: Vec<StateID>,
    /// Sparse maps ids to their location in dense.
    ///
    /// A state ID is in the set if and only if
    /// sparse[id] < len && id == dense[sparse[id]].
    ///
    /// Note that these are indices into 'dense'. It's a little weird to use
    /// StateID here, but we know our length can never exceed the bounds of
    /// StateID (enforced by 'resize') and StateID will be at most 4 bytes
    /// where as a usize is likely double that in most cases.
    sparse: Vec<StateID>,
}
#[derive(Clone)]
pub struct Input<'h> {
    haystack: &'h [u8],
    span: Span,
    anchored: Anchored,
    earliest: bool,
}
#[derive(Clone, Debug, Default)]
pub struct Config {
    match_kind: Option<MatchKind>,
    pre: Option<Option<Prefilter>>,
}
#[derive(Clone, Debug)]
pub struct Prefilter {
    #[cfg(not(feature = "alloc"))]
    _unused: (),
    #[cfg(feature = "alloc")]
    pre: Arc<dyn PrefilterI>,
    #[cfg(feature = "alloc")]
    is_fast: bool,
    #[cfg(feature = "alloc")]
    max_needle_len: usize,
}
#[derive(Clone, Debug)]
struct SlotTable {
    /// The actual table of offsets.
    table: Vec<Option<NonMaxUsize>>,
    /// The number of slots per state, i.e., the table's stride or the length
    /// of each row.
    slots_per_state: usize,
    /// The number of slots in the caller-provided 'Captures' value for the
    /// current search. Setting this to 'slots_per_state' is always correct,
    /// but may be wasteful.
    slots_for_captures: usize,
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
#[derive(Clone, Debug)]
pub struct Config {
    look_behind: Option<u8>,
    anchored: Anchored,
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
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct NonMaxUsize(NonZeroUsize);
#[derive(Clone)]
pub struct NFA(Arc<Inner>);
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
#[derive(Clone, Debug)]
pub(crate) struct Config {
    match_kind: MatchKind,
    quit: ByteSet,
    dfa_size_limit: Option<usize>,
    determinize_size_limit: Option<usize>,
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
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Span {
    /// The start offset of the span, inclusive.
    pub start: usize,
    /// The end offset of the span, exclusive.
    pub end: usize,
}
#[derive(Clone, Debug)]
struct ActiveStates {
    /// The set of active NFA states. This set preserves insertion order, which
    /// is critical for simulating the match semantics of backtracking regex
    /// engines.
    set: SparseSet,
    /// The slots for every NFA state, where each slot stores a (possibly
    /// absent) offset. Every capturing group has two slots. One for a start
    /// offset and one for an end offset.
    slot_table: SlotTable,
}
#[derive(Clone, Debug, Default)]
pub struct Config {
    match_kind: Option<MatchKind>,
    starts_for_each_pattern: Option<bool>,
    byte_classes: Option<bool>,
    size_limit: Option<Option<usize>>,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
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
#[derive(Clone, Debug, Default)]
pub struct Config {
    pre: Option<Option<Prefilter>>,
    visited_capacity: Option<usize>,
}
#[non_exhaustive]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MatchKind {
    /// Report all possible matches.
    All,
    /// Report only the leftmost matches. When multiple leftmost matches exist,
    /// report the match corresponding to the part of the regex that appears
    /// first in the syntax.
    LeftmostFirst,
}
#[derive(Clone, Debug)]
enum FollowEpsilon {
    /// Explore the epsilon transitions from a state ID.
    Explore(StateID),
    /// Reset the given `slot` to the given `offset` (which might be `None`).
    RestoreCapture { slot: SmallIndex, offset: Option<NonMaxUsize> },
}
impl PikeVM {
    fn search_imp(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Option<HalfMatch> {
        cache.setup_search(slots.len());
        if input.is_done() {
            return None;
        }
        assert!(
            input.haystack().len() < core::usize::MAX,
            "byte slice lengths must be less than usize MAX",
        );
        instrument!(| c | c.reset(& self.nfa));
        let allmatches = self.config.get_match_kind().continue_past_first_match();
        let (anchored, start_id) = match self.start_config(input) {
            None => return None,
            Some(config) => config,
        };
        let pre = if anchored { None } else { self.get_config().get_prefilter() };
        let Cache { ref mut stack, ref mut curr, ref mut next } = cache;
        let mut hm = None;
        let mut at = input.start();
        while at <= input.end() {
            if curr.set.is_empty() {
                if hm.is_some() && !allmatches {
                    break;
                }
                if anchored && at > input.start() {
                    break;
                }
                if let Some(ref pre) = pre {
                    let span = Span::from(at..input.end());
                    match pre.find(input.haystack(), span) {
                        None => break,
                        Some(ref span) => at = span.start,
                    }
                }
            }
            if (!hm.is_some() || allmatches) && (!anchored || at == input.start()) {
                let slots = next.slot_table.all_absent();
                self.epsilon_closure(stack, slots, curr, input, at, start_id);
            }
            if let Some(pid) = self.nexts(stack, curr, next, input, at, slots) {
                hm = Some(HalfMatch::new(pid, at));
            }
            if input.get_earliest() && hm.is_some() {
                break;
            }
            core::mem::swap(curr, next);
            next.set.clear();
            at += 1;
        }
        instrument!(| c | c.eprint(& self.nfa));
        hm
    }
    fn which_overlapping_imp(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        patset: &mut PatternSet,
    ) {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn nexts(
        &self,
        stack: &mut Vec<FollowEpsilon>,
        curr: &mut ActiveStates,
        next: &mut ActiveStates,
        input: &Input<'_>,
        at: usize,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Option<PatternID> {
        instrument!(| c | c.record_state_set(& curr.set));
        let mut pid = None;
        let ActiveStates { ref set, ref mut slot_table } = *curr;
        for sid in set.iter() {
            pid = match self.next(stack, slot_table, next, input, at, sid) {
                None => continue,
                Some(pid) => Some(pid),
            };
            slots.copy_from_slice(slot_table.for_state(sid));
            if !self.config.get_match_kind().continue_past_first_match() {
                break;
            }
        }
        pid
    }
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn nexts_overlapping(
        &self,
        stack: &mut Vec<FollowEpsilon>,
        curr: &mut ActiveStates,
        next: &mut ActiveStates,
        input: &Input<'_>,
        at: usize,
        patset: &mut PatternSet,
    ) {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn next(
        &self,
        stack: &mut Vec<FollowEpsilon>,
        curr_slot_table: &mut SlotTable,
        next: &mut ActiveStates,
        input: &Input<'_>,
        at: usize,
        sid: StateID,
    ) -> Option<PatternID> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn epsilon_closure(
        &self,
        stack: &mut Vec<FollowEpsilon>,
        curr_slots: &mut [Option<NonMaxUsize>],
        next: &mut ActiveStates,
        input: &Input<'_>,
        at: usize,
        sid: StateID,
    ) {
        instrument!(| c | { c.record_closure(sid); c.record_stack_push(sid); });
        stack.push(FollowEpsilon::Explore(sid));
        while let Some(frame) = stack.pop() {
            match frame {
                FollowEpsilon::RestoreCapture { slot, offset: pos } => {
                    curr_slots[slot] = pos;
                }
                FollowEpsilon::Explore(sid) => {
                    self.epsilon_closure_explore(
                        stack,
                        curr_slots,
                        next,
                        input,
                        at,
                        sid,
                    );
                }
            }
        }
    }
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn epsilon_closure_explore(
        &self,
        stack: &mut Vec<FollowEpsilon>,
        curr_slots: &mut [Option<NonMaxUsize>],
        next: &mut ActiveStates,
        input: &Input<'_>,
        at: usize,
        mut sid: StateID,
    ) {}
    fn start_config(&self, input: &Input<'_>) -> Option<(bool, StateID)> {
        match input.get_anchored() {
            Anchored::No => {
                Some((self.nfa.is_always_start_anchored(), self.nfa.start_anchored()))
            }
            Anchored::Yes => Some((true, self.nfa.start_anchored())),
            Anchored::Pattern(pid) => Some((true, self.nfa.start_pattern(pid)?)),
        }
    }
}
impl SparseSet {
    #[inline]
    pub(crate) fn new(capacity: usize) -> SparseSet {}
    #[inline]
    pub(crate) fn resize(&mut self, new_capacity: usize) {}
    #[inline]
    pub(crate) fn capacity(&self) -> usize {}
    #[inline]
    pub(crate) fn len(&self) -> usize {}
    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        self.len() == 0
    }
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn insert(&mut self, id: StateID) -> bool {}
    #[inline]
    pub(crate) fn contains(&self, id: StateID) -> bool {}
    #[inline]
    pub(crate) fn clear(&mut self) {
        self.len = 0;
    }
    #[inline]
    pub(crate) fn iter(&self) -> SparseSetIter<'_> {}
    #[inline]
    pub(crate) fn memory_usage(&self) -> usize {}
}
impl<'h> Input<'h> {
    #[inline]
    pub fn new<H: ?Sized + AsRef<[u8]>>(haystack: &'h H) -> Input<'h> {}
    #[inline]
    pub fn span<S: Into<Span>>(mut self, span: S) -> Input<'h> {}
    #[inline]
    pub fn range<R: RangeBounds<usize>>(mut self, range: R) -> Input<'h> {}
    #[inline]
    pub fn anchored(mut self, mode: Anchored) -> Input<'h> {}
    #[inline]
    pub fn earliest(mut self, yes: bool) -> Input<'h> {}
    #[inline]
    pub fn set_span<S: Into<Span>>(&mut self, span: S) {}
    #[inline]
    pub fn set_range<R: RangeBounds<usize>>(&mut self, range: R) {}
    #[inline]
    pub fn set_start(&mut self, start: usize) {}
    #[inline]
    pub fn set_end(&mut self, end: usize) {}
    #[inline]
    pub fn set_anchored(&mut self, mode: Anchored) {}
    #[inline]
    pub fn set_earliest(&mut self, yes: bool) {}
    #[inline]
    pub fn haystack(&self) -> &[u8] {
        self.haystack
    }
    #[inline]
    pub fn start(&self) -> usize {
        self.get_span().start
    }
    #[inline]
    pub fn end(&self) -> usize {
        self.get_span().end
    }
    #[inline]
    pub fn get_span(&self) -> Span {}
    #[inline]
    pub fn get_range(&self) -> Range<usize> {}
    #[inline]
    pub fn get_anchored(&self) -> Anchored {}
    #[inline]
    pub fn get_earliest(&self) -> bool {
        self.earliest
    }
    #[inline]
    pub fn is_done(&self) -> bool {
        self.get_span().start > self.get_span().end
    }
    #[inline]
    pub fn is_char_boundary(&self, offset: usize) -> bool {}
}
impl Config {
    pub fn new() -> Config {}
    pub fn match_kind(mut self, kind: MatchKind) -> Config {}
    pub fn prefilter(mut self, pre: Option<Prefilter>) -> Config {}
    pub fn get_match_kind(&self) -> MatchKind {
        self.match_kind.unwrap_or(MatchKind::LeftmostFirst)
    }
    pub fn get_prefilter(&self) -> Option<&Prefilter> {
        self.pre.as_ref().unwrap_or(&None).as_ref()
    }
    pub(crate) fn overwrite(&self, o: Config) -> Config {}
}
impl Prefilter {
    pub fn new<B: AsRef<[u8]>>(kind: MatchKind, needles: &[B]) -> Option<Prefilter> {}
    fn from_choice(choice: Choice, max_needle_len: usize) -> Option<Prefilter> {}
    #[cfg(feature = "syntax")]
    pub fn from_hir_prefix(kind: MatchKind, hir: &Hir) -> Option<Prefilter> {}
    #[cfg(feature = "syntax")]
    pub fn from_hirs_prefix<H: Borrow<Hir>>(
        kind: MatchKind,
        hirs: &[H],
    ) -> Option<Prefilter> {}
    #[inline]
    pub fn find(&self, haystack: &[u8], span: Span) -> Option<Span> {
        #[cfg(not(feature = "alloc"))] { unreachable!() }
        #[cfg(feature = "alloc")] { self.pre.find(haystack, span) }
    }
    #[inline]
    pub fn prefix(&self, haystack: &[u8], span: Span) -> Option<Span> {}
    #[inline]
    pub fn memory_usage(&self) -> usize {}
    #[inline]
    pub fn max_needle_len(&self) -> usize {}
    #[inline]
    pub fn is_fast(&self) -> bool {}
}
impl SlotTable {
    fn new() -> SlotTable {}
    fn reset(&mut self, re: &PikeVM) {}
    fn memory_usage(&self) -> usize {}
    fn setup_search(&mut self, captures_slot_len: usize) {}
    fn for_state(&mut self, sid: StateID) -> &mut [Option<NonMaxUsize>] {}
    fn all_absent(&mut self) -> &mut [Option<NonMaxUsize>] {
        let i = self.table.len() - self.slots_for_captures;
        &mut self.table[i..i + self.slots_for_captures]
    }
}
impl Cache {
    pub fn new(re: &PikeVM) -> Cache {}
    pub fn reset(&mut self, re: &PikeVM) {}
    pub fn memory_usage(&self) -> usize {}
    fn setup_search(&mut self, captures_slot_len: usize) {
        self.stack.clear();
        self.curr.setup_search(captures_slot_len);
        self.next.setup_search(captures_slot_len);
    }
}
impl MatchKind {
    #[cfg(feature = "alloc")]
    pub(crate) fn continue_past_first_match(&self) -> bool {
        *self == MatchKind::All
    }
}
impl HalfMatch {
    #[inline]
    pub fn new(pattern: PatternID, offset: usize) -> HalfMatch {
        HalfMatch { pattern, offset }
    }
    #[inline]
    pub fn must(pattern: usize, offset: usize) -> HalfMatch {}
    #[inline]
    pub fn pattern(&self) -> PatternID {}
    #[inline]
    pub fn offset(&self) -> usize {}
}
