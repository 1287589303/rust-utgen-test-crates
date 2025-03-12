use alloc::{vec, vec::Vec};
use crate::{
    dfa::{remapper::Remapper, DEAD},
    nfa::thompson::{self, NFA},
    util::{
        alphabet::ByteClasses, captures::Captures, escape::DebugByte,
        int::{Usize, U32, U64, U8},
        look::{Look, LookSet, UnicodeWordBoundaryError},
        primitives::{NonMaxUsize, PatternID, StateID},
        search::{Anchored, Input, Match, MatchError, MatchKind, Span},
        sparse_set::SparseSet,
    },
};
pub(super) trait Remappable: core::fmt::Debug {
    fn state_len(&self) -> usize;
    fn stride2(&self) -> usize;
    fn swap_states(&mut self, id1: StateID, id2: StateID);
    fn remap(&mut self, map: impl Fn(StateID) -> StateID);
}
#[derive(Clone)]
pub struct DFA {
    /// The configuration provided by the caller.
    config: Config,
    /// The NFA used to build this DFA.
    ///
    /// NOTE: We probably don't need to store the NFA here, but we use enough
    /// bits from it that it's convenient to do so. And there really isn't much
    /// cost to doing so either, since an NFA is reference counted internally.
    nfa: NFA,
    /// The transition table. Given a state ID 's' and a byte of haystack 'b',
    /// the next state is `table[sid + classes[byte]]`.
    ///
    /// The stride of this table (i.e., the number of columns) is always
    /// a power of 2, even if the alphabet length is smaller. This makes
    /// converting between state IDs and state indices very cheap.
    ///
    /// Note that the stride always includes room for one extra "transition"
    /// that isn't actually a transition. It is a 'PatternEpsilons' that is
    /// used for match states only. Because of this, the maximum number of
    /// active columns in the transition table is 257, which means the maximum
    /// stride is 512 (the next power of 2 greater than or equal to 257).
    table: Vec<Transition>,
    /// The DFA state IDs of the starting states.
    ///
    /// `starts[0]` is always present and corresponds to the starting state
    /// when searching for matches of any pattern in the DFA.
    ///
    /// `starts[i]` where i>0 corresponds to the starting state for the pattern
    /// ID 'i-1'. These starting states are optional.
    starts: Vec<StateID>,
    /// Every state ID >= this value corresponds to a match state.
    ///
    /// This is what a search uses to detect whether a state is a match state
    /// or not. It requires only a simple comparison instead of bit-unpacking
    /// the PatternEpsilons from every state.
    min_match_id: StateID,
    /// The alphabet of this DFA, split into equivalence classes. Bytes in the
    /// same equivalence class can never discriminate between a match and a
    /// non-match.
    classes: ByteClasses,
    /// The number of elements in each state in the transition table. This may
    /// be less than the stride, since the stride is always a power of 2 and
    /// the alphabet length can be anything up to and including 256.
    alphabet_len: usize,
    /// The number of columns in the transition table, expressed as a power of
    /// 2.
    stride2: usize,
    /// The offset at which the PatternEpsilons for a match state is stored in
    /// the transition table.
    ///
    /// PERF: One wonders whether it would be better to put this in a separate
    /// allocation, since only match states have a non-empty PatternEpsilons
    /// and the number of match states tends be dwarfed by the number of
    /// non-match states. So this would save '8*len(non_match_states)' for each
    /// DFA. The question is whether moving this to a different allocation will
    /// lead to a perf hit during searches. You might think dealing with match
    /// states is rare, but some regexes spend a lot of time in match states
    /// gobbling up input. But... match state handling is already somewhat
    /// expensive, so maybe this wouldn't do much? Either way, it's worth
    /// experimenting.
    pateps_offset: usize,
    /// The first explicit slot index. This refers to the first slot appearing
    /// immediately after the last implicit slot. It is always 'patterns.len()
    /// * 2'.
    ///
    /// We record this because we only store the explicit slots in our DFA
    /// transition table that need to be saved. Implicit slots are handled
    /// automatically as part of the search.
    explicit_slot_start: usize,
}
#[derive(Clone, Debug)]
pub struct Cache {
    /// Scratch space used to store slots during a search. Basically, we use
    /// the caller provided slots to store slots known when a match occurs.
    /// But after a match occurs, we might continue a search but ultimately
    /// fail to extend the match. When continuing the search, we need some
    /// place to store candidate capture offsets without overwriting the slot
    /// offsets recorded for the most recently seen match.
    explicit_slots: Vec<Option<NonMaxUsize>>,
    /// The number of slots in the caller-provided 'Captures' value for the
    /// current search. This is always at most 'explicit_slots.len()', but
    /// might be less than it, if the caller provided fewer slots to fill.
    explicit_slot_len: usize,
}
#[derive(Clone, Debug)]
pub struct LookMatcher {
    lineterm: DebugByte,
}
#[derive(Clone)]
pub struct Input<'h> {
    haystack: &'h [u8],
    span: Span,
    anchored: Anchored,
    earliest: bool,
}
#[derive(Clone)]
pub struct NFA(Arc<Inner>);
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchError(
    #[cfg(feature = "alloc")]
    alloc::boxed::Box<MatchErrorKind>,
    #[cfg(not(feature = "alloc"))]
    MatchErrorKind,
);
#[derive(Clone, Copy, Eq, PartialEq)]
struct Transition(u64);
#[derive(Clone, Copy)]
struct Slots(u32);
#[derive(Clone, Copy)]
struct Epsilons(u64);
#[derive(Clone, Debug, Default)]
pub struct Config {
    match_kind: Option<MatchKind>,
    starts_for_each_pattern: Option<bool>,
    byte_classes: Option<bool>,
    size_limit: Option<Option<usize>>,
}
#[derive(Clone, Copy, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct NonMaxUsize(NonZeroUsize);
#[derive(Clone, Copy, Default, Eq, PartialEq)]
pub struct LookSet {
    /// The underlying representation this set is exposed to make it possible
    /// to store it somewhere efficiently. The representation is that
    /// of a bitset, where each assertion occupies bit `i` where
    /// `i = Look::as_repr()`.
    ///
    /// Note that users of this internal representation must permit the full
    /// range of `u16` values to be represented. For example, even if the
    /// current implementation only makes use of the 10 least significant bits,
    /// it may use more bits in a future semver compatible release.
    pub bits: u32,
}
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Transition {
    /// The inclusive start of the byte range.
    pub start: u8,
    /// The inclusive end of the byte range.
    pub end: u8,
    /// The identifier of the state to transition to.
    pub next: StateID,
}
#[derive(Debug)]
pub struct PatternIter<'a> {
    it: PatternIDIter,
    /// We explicitly associate a lifetime with this iterator even though we
    /// don't actually borrow anything from the NFA. We do this for backward
    /// compatibility purposes. If we ever do need to borrow something from
    /// the NFA, then we can and just get rid of this marker without breaking
    /// the public API.
    _marker: core::marker::PhantomData<&'a ()>,
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
    match_kind: Option<MatchKind>,
    pre: Option<Option<Prefilter>>,
}
#[derive(Clone)]
struct Transition {
    /// The byte range.
    range: Utf8Range,
    /// The next state to transition to.
    next_id: StateID,
}
#[derive(Clone, Debug)]
pub struct Config {
    look_behind: Option<u8>,
    anchored: Anchored,
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
#[derive(Clone, Copy)]
pub struct ByteClasses([u8; 256]);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct StateID(SmallIndex);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
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
#[derive(Clone, Copy)]
struct Transition {
    byte: u8,
    next: StateID,
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
#[derive(Clone, Debug, Default)]
pub struct Config {
    pre: Option<Option<Prefilter>>,
    visited_capacity: Option<usize>,
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
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Anchored {
    /// Run an unanchored search. This means a match may occur anywhere at or
    /// after the start position of the search.
    ///
    /// This search can return a match for any pattern in the regex.
    No,
    /// Run an anchored search. This means that a match must begin at the
    /// start position of the search.
    ///
    /// This search can return a match for any pattern in the regex.
    Yes,
    /// Run an anchored search for a specific pattern. This means that a match
    /// must be for the given pattern and must begin at the start position of
    /// the search.
    Pattern(PatternID),
}
impl DFA {
    fn search_imp(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        slots: &mut [Option<NonMaxUsize>],
    ) -> Result<Option<PatternID>, MatchError> {
        if input.is_done() {
            return Ok(None);
        }
        let explicit_slots_len = core::cmp::min(
            Slots::LIMIT,
            slots.len().saturating_sub(self.explicit_slot_start),
        );
        cache.setup_search(explicit_slots_len);
        for slot in cache.explicit_slots() {
            *slot = None;
        }
        for slot in slots.iter_mut() {
            *slot = None;
        }
        for pid in self.nfa.patterns() {
            let i = pid.as_usize() * 2;
            if i >= slots.len() {
                break;
            }
            slots[i] = NonMaxUsize::new(input.start());
        }
        let mut pid = None;
        let mut next_sid = match input.get_anchored() {
            Anchored::Yes => self.start(),
            Anchored::Pattern(pid) => self.start_pattern(pid)?,
            Anchored::No => {
                if !self.nfa.is_always_start_anchored() {
                    return Err(MatchError::unsupported_anchored(Anchored::No));
                }
                self.start()
            }
        };
        let leftmost_first = matches!(
            self.config.get_match_kind(), MatchKind::LeftmostFirst
        );
        for at in input.start()..input.end() {
            let sid = next_sid;
            let trans = self.transition(sid, input.haystack()[at]);
            next_sid = trans.state_id();
            let epsilons = trans.epsilons();
            if sid >= self.min_match_id {
                if self.find_match(cache, input, at, sid, slots, &mut pid) {
                    if input.get_earliest() || (leftmost_first && trans.match_wins()) {
                        return Ok(pid);
                    }
                }
            }
            if sid == DEAD
                || (!epsilons.looks().is_empty()
                    && !self
                        .nfa
                        .look_matcher()
                        .matches_set_inline(epsilons.looks(), input.haystack(), at))
            {
                return Ok(pid);
            }
            epsilons.slots().apply(at, cache.explicit_slots());
        }
        if next_sid >= self.min_match_id {
            self.find_match(cache, input, input.end(), next_sid, slots, &mut pid);
        }
        Ok(pid)
    }
    #[cfg_attr(feature = "perf-inline", inline(always))]
    fn find_match(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
        at: usize,
        sid: StateID,
        slots: &mut [Option<NonMaxUsize>],
        matched_pid: &mut Option<PatternID>,
    ) -> bool {
        debug_assert!(sid >= self.min_match_id);
        let pateps = self.pattern_epsilons(sid);
        let epsilons = pateps.epsilons();
        if !epsilons.looks().is_empty()
            && !self
                .nfa
                .look_matcher()
                .matches_set_inline(epsilons.looks(), input.haystack(), at)
        {
            return false;
        }
        let pid = pateps.pattern_id_unchecked();
        let slot_end = pid.as_usize().wrapping_mul(2).wrapping_add(1);
        if slot_end < slots.len() {
            slots[slot_end] = NonMaxUsize::new(at);
        }
        if self.explicit_slot_start < slots.len() {
            slots[self.explicit_slot_start..].copy_from_slice(cache.explicit_slots());
            epsilons.slots().apply(at, &mut slots[self.explicit_slot_start..]);
        }
        *matched_pid = Some(pid);
        true
    }
}
impl Cache {
    pub fn new(re: &DFA) -> Cache {}
    pub fn reset(&mut self, re: &DFA) {}
    pub fn memory_usage(&self) -> usize {}
    fn explicit_slots(&mut self) -> &mut [Option<NonMaxUsize>] {
        &mut self.explicit_slots[..self.explicit_slot_len]
    }
    fn setup_search(&mut self, explicit_slot_len: usize) {
        self.explicit_slot_len = explicit_slot_len;
    }
}
impl LookMatcher {
    pub fn new() -> LookMatcher {}
    pub fn set_line_terminator(&mut self, byte: u8) -> &mut LookMatcher {}
    pub fn get_line_terminator(&self) -> u8 {}
    #[inline]
    pub fn matches(&self, look: Look, haystack: &[u8], at: usize) -> bool {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn matches_inline(&self, look: Look, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn matches_set(&self, set: LookSet, haystack: &[u8], at: usize) -> bool {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn matches_set_inline(
        &self,
        set: LookSet,
        haystack: &[u8],
        at: usize,
    ) -> bool {
        if set.contains(Look::Start) {
            if !self.is_start(haystack, at) {
                return false;
            }
        }
        if set.contains(Look::End) {
            if !self.is_end(haystack, at) {
                return false;
            }
        }
        if set.contains(Look::StartLF) {
            if !self.is_start_lf(haystack, at) {
                return false;
            }
        }
        if set.contains(Look::EndLF) {
            if !self.is_end_lf(haystack, at) {
                return false;
            }
        }
        if set.contains(Look::StartCRLF) {
            if !self.is_start_crlf(haystack, at) {
                return false;
            }
        }
        if set.contains(Look::EndCRLF) {
            if !self.is_end_crlf(haystack, at) {
                return false;
            }
        }
        if set.contains(Look::WordAscii) {
            if !self.is_word_ascii(haystack, at) {
                return false;
            }
        }
        if set.contains(Look::WordAsciiNegate) {
            if !self.is_word_ascii_negate(haystack, at) {
                return false;
            }
        }
        if set.contains(Look::WordUnicode) {
            if !self.is_word_unicode(haystack, at).unwrap() {
                return false;
            }
        }
        if set.contains(Look::WordUnicodeNegate) {
            if !self.is_word_unicode_negate(haystack, at).unwrap() {
                return false;
            }
        }
        if set.contains(Look::WordStartAscii) {
            if !self.is_word_start_ascii(haystack, at) {
                return false;
            }
        }
        if set.contains(Look::WordEndAscii) {
            if !self.is_word_end_ascii(haystack, at) {
                return false;
            }
        }
        if set.contains(Look::WordStartUnicode) {
            if !self.is_word_start_unicode(haystack, at).unwrap() {
                return false;
            }
        }
        if set.contains(Look::WordEndUnicode) {
            if !self.is_word_end_unicode(haystack, at).unwrap() {
                return false;
            }
        }
        if set.contains(Look::WordStartHalfAscii) {
            if !self.is_word_start_half_ascii(haystack, at) {
                return false;
            }
        }
        if set.contains(Look::WordEndHalfAscii) {
            if !self.is_word_end_half_ascii(haystack, at) {
                return false;
            }
        }
        if set.contains(Look::WordStartHalfUnicode) {
            if !self.is_word_start_half_unicode(haystack, at).unwrap() {
                return false;
            }
        }
        if set.contains(Look::WordEndHalfUnicode) {
            if !self.is_word_end_half_unicode(haystack, at).unwrap() {
                return false;
            }
        }
        true
    }
    #[cfg(feature = "alloc")]
    pub(crate) fn add_to_byteset(
        &self,
        look: Look,
        set: &mut crate::util::alphabet::ByteClassSet,
    ) {}
    #[inline]
    pub fn is_start(&self, _haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_end(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_start_lf(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_end_lf(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_start_crlf(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_end_crlf(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_word_ascii(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_word_ascii_negate(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_word_unicode(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, UnicodeWordBoundaryError> {}
    #[inline]
    pub fn is_word_unicode_negate(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, UnicodeWordBoundaryError> {}
    #[inline]
    pub fn is_word_start_ascii(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_word_end_ascii(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_word_start_unicode(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, UnicodeWordBoundaryError> {}
    #[inline]
    pub fn is_word_end_unicode(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, UnicodeWordBoundaryError> {}
    #[inline]
    pub fn is_word_start_half_ascii(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_word_end_half_ascii(&self, haystack: &[u8], at: usize) -> bool {}
    #[inline]
    pub fn is_word_start_half_unicode(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, UnicodeWordBoundaryError> {}
    #[inline]
    pub fn is_word_end_half_unicode(
        &self,
        haystack: &[u8],
        at: usize,
    ) -> Result<bool, UnicodeWordBoundaryError> {}
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
    pub fn get_anchored(&self) -> Anchored {
        self.anchored
    }
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
    pub fn patterns(&self) -> PatternIter<'_> {
        PatternIter {
            it: PatternID::iter(self.pattern_len()),
            _marker: core::marker::PhantomData,
        }
    }
    #[inline]
    pub fn pattern_len(&self) -> usize {}
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
    pub fn group_info(&self) -> &GroupInfo {}
    #[inline]
    pub fn has_capture(&self) -> bool {}
    #[inline]
    pub fn has_empty(&self) -> bool {}
    #[inline]
    pub fn is_utf8(&self) -> bool {}
    #[inline]
    pub fn is_reverse(&self) -> bool {}
    #[inline]
    pub fn is_always_start_anchored(&self) -> bool {
        self.start_anchored() == self.start_unanchored()
    }
    #[inline]
    pub fn look_matcher(&self) -> &LookMatcher {
        &self.0.look_matcher
    }
    #[inline]
    pub fn look_set_any(&self) -> LookSet {}
    #[inline]
    pub fn look_set_prefix_any(&self) -> LookSet {}
    #[inline]
    pub fn memory_usage(&self) -> usize {}
}
impl MatchError {
    pub fn new(kind: MatchErrorKind) -> MatchError {}
    pub fn kind(&self) -> &MatchErrorKind {}
    pub fn quit(byte: u8, offset: usize) -> MatchError {}
    pub fn gave_up(offset: usize) -> MatchError {}
    pub fn haystack_too_long(len: usize) -> MatchError {}
    pub fn unsupported_anchored(mode: Anchored) -> MatchError {
        MatchError::new(MatchErrorKind::UnsupportedAnchored {
            mode,
        })
    }
}
impl Transition {
    const STATE_ID_BITS: u64 = 21;
    const STATE_ID_SHIFT: u64 = 64 - Transition::STATE_ID_BITS;
    const STATE_ID_LIMIT: u64 = 1 << Transition::STATE_ID_BITS;
    const MATCH_WINS_SHIFT: u64 = 64 - (Transition::STATE_ID_BITS + 1);
    const INFO_MASK: u64 = 0x000003FF_FFFFFFFF;
    fn new(match_wins: bool, sid: StateID, epsilons: Epsilons) -> Transition {}
    fn is_dead(self) -> bool {}
    fn match_wins(&self) -> bool {
        (self.0 >> Transition::MATCH_WINS_SHIFT & 1) == 1
    }
    fn state_id(&self) -> StateID {
        StateID::new_unchecked((self.0 >> Transition::STATE_ID_SHIFT).as_usize())
    }
    fn set_state_id(&mut self, sid: StateID) {}
    fn epsilons(&self) -> Epsilons {
        Epsilons(self.0 & Transition::INFO_MASK)
    }
}
impl Slots {
    const LIMIT: usize = 32;
    fn insert(self, slot: usize) -> Slots {}
    fn remove(self, slot: usize) -> Slots {}
    fn is_empty(self) -> bool {}
    fn iter(self) -> SlotsIter {}
    fn apply(self, at: usize, caller_explicit_slots: &mut [Option<NonMaxUsize>]) {
        if self.is_empty() {
            return;
        }
        let at = NonMaxUsize::new(at);
        for slot in self.iter() {
            if slot >= caller_explicit_slots.len() {
                break;
            }
            caller_explicit_slots[slot] = at;
        }
    }
}
impl Epsilons {
    const SLOT_MASK: u64 = 0x000003FF_FFFFFC00;
    const SLOT_SHIFT: u64 = 10;
    const LOOK_MASK: u64 = 0x00000000_000003FF;
    fn empty() -> Epsilons {}
    fn is_empty(self) -> bool {}
    fn slots(self) -> Slots {
        Slots((self.0 >> Epsilons::SLOT_SHIFT).low_u32())
    }
    fn set_slots(self, slots: Slots) -> Epsilons {}
    fn looks(self) -> LookSet {
        LookSet {
            bits: (self.0 & Epsilons::LOOK_MASK).low_u32(),
        }
    }
    fn set_looks(self, look_set: LookSet) -> Epsilons {}
}
impl Config {
    pub fn new() -> Config {}
    pub fn match_kind(mut self, kind: MatchKind) -> Config {}
    pub fn starts_for_each_pattern(mut self, yes: bool) -> Config {}
    pub fn byte_classes(mut self, yes: bool) -> Config {}
    pub fn size_limit(mut self, limit: Option<usize>) -> Config {}
    pub fn get_match_kind(&self) -> MatchKind {
        self.match_kind.unwrap_or(MatchKind::LeftmostFirst)
    }
    pub fn get_starts_for_each_pattern(&self) -> bool {}
    pub fn get_byte_classes(&self) -> bool {}
    pub fn get_size_limit(&self) -> Option<usize> {}
    pub(crate) fn overwrite(&self, o: Config) -> Config {}
}
impl NonMaxUsize {
    #[inline]
    pub fn new(value: usize) -> Option<NonMaxUsize> {
        NonZeroUsize::new(value.wrapping_add(1)).map(NonMaxUsize)
    }
    #[inline]
    pub fn get(self) -> usize {}
}
impl LookSet {
    #[inline]
    pub fn empty() -> LookSet {}
    #[inline]
    pub fn full() -> LookSet {}
    #[inline]
    pub fn singleton(look: Look) -> LookSet {}
    #[inline]
    pub fn len(self) -> usize {}
    #[inline]
    pub fn is_empty(self) -> bool {
        self.len() == 0
    }
    #[inline]
    pub fn contains(self, look: Look) -> bool {}
    #[inline]
    pub fn contains_anchor(&self) -> bool {}
    #[inline]
    pub fn contains_anchor_haystack(&self) -> bool {}
    #[inline]
    pub fn contains_anchor_line(&self) -> bool {}
    #[inline]
    pub fn contains_anchor_lf(&self) -> bool {}
    #[inline]
    pub fn contains_anchor_crlf(&self) -> bool {}
    #[inline]
    pub fn contains_word(self) -> bool {}
    #[inline]
    pub fn contains_word_unicode(self) -> bool {}
    #[inline]
    pub fn contains_word_ascii(self) -> bool {}
    #[inline]
    pub fn iter(self) -> LookSetIter {}
    #[inline]
    pub fn insert(self, look: Look) -> LookSet {}
    #[inline]
    pub fn set_insert(&mut self, look: Look) {}
    #[inline]
    pub fn remove(self, look: Look) -> LookSet {}
    #[inline]
    pub fn set_remove(&mut self, look: Look) {}
    #[inline]
    pub fn subtract(self, other: LookSet) -> LookSet {}
    #[inline]
    pub fn set_subtract(&mut self, other: LookSet) {}
    #[inline]
    pub fn union(self, other: LookSet) -> LookSet {}
    #[inline]
    pub fn set_union(&mut self, other: LookSet) {}
    #[inline]
    pub fn intersect(self, other: LookSet) -> LookSet {}
    #[inline]
    pub fn set_intersect(&mut self, other: LookSet) {}
    #[inline]
    pub fn read_repr(slice: &[u8]) -> LookSet {}
    #[inline]
    pub fn write_repr(self, slice: &mut [u8]) {}
    pub fn available(self) -> Result<(), UnicodeWordBoundaryError> {}
}
