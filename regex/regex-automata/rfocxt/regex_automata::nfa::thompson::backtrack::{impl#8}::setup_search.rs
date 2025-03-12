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
struct Visited {
    /// The actual underlying bitset. Each element in the bitset corresponds
    /// to a particular (StateID, offset) pair. States correspond to the rows
    /// and the offsets correspond to the columns.
    ///
    /// If our underlying NFA has N states and the haystack we're searching
    /// has M bytes, then we have N*(M+1) entries in our bitset table. The
    /// M+1 occurs because our matches are delayed by one byte (to support
    /// look-around), and so we need to handle the end position itself rather
    /// than stopping just before the end. (If there is no end position, then
    /// it's treated as "end-of-input," which is matched by things like '$'.)
    ///
    /// Given BITS=N*(M+1), we wind up with div_ceil(BITS, sizeof(usize))
    /// blocks.
    ///
    /// We use 'usize' to represent our blocks because it makes some of the
    /// arithmetic in 'insert' a bit nicer. For example, if we used 'u32' for
    /// our block, we'd either need to cast u32s to usizes or usizes to u32s.
    bitset: Vec<usize>,
    /// The stride represents one plus length of the haystack we're searching
    /// (as described above). The stride must be initialized for each search.
    stride: usize,
}
#[derive(Clone, Debug)]
pub struct BoundedBacktracker {
    config: Config,
    nfa: NFA,
}
#[derive(Clone, Debug, Default)]
pub struct Config {
    pre: Option<Option<Prefilter>>,
    visited_capacity: Option<usize>,
}
#[derive(Clone)]
pub struct Input<'h> {
    haystack: &'h [u8],
    span: Span,
    anchored: Anchored,
    earliest: bool,
}
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Span {
    /// The start offset of the span, inclusive.
    pub start: usize,
    /// The end offset of the span, exclusive.
    pub end: usize,
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
#[derive(Clone, Eq, PartialEq)]
pub enum State {
    /// A state with a single transition that can only be taken if the current
    /// input symbol is in a particular range of bytes.
    ByteRange {
        /// The transition from this state to the next.
        trans: Transition,
    },
    /// A state with possibly many transitions represented in a sparse fashion.
    /// Transitions are non-overlapping and ordered lexicographically by input
    /// range.
    ///
    /// In practice, this is used for encoding UTF-8 automata. Its presence is
    /// primarily an optimization that avoids many additional unconditional
    /// epsilon transitions (via [`Union`](State::Union) states), and thus
    /// decreases the overhead of traversing the NFA. This can improve both
    /// matching time and DFA construction time.
    Sparse(SparseTransitions),
    /// A dense representation of a state with multiple transitions.
    Dense(DenseTransitions),
    /// A conditional epsilon transition satisfied via some sort of
    /// look-around. Look-around is limited to anchor and word boundary
    /// assertions.
    ///
    /// Look-around states are meant to be evaluated while performing epsilon
    /// closure (computing the set of states reachable from a particular state
    /// via only epsilon transitions). If the current position in the haystack
    /// satisfies the look-around assertion, then you're permitted to follow
    /// that epsilon transition.
    Look {
        /// The look-around assertion that must be satisfied before moving
        /// to `next`.
        look: Look,
        /// The state to transition to if the look-around assertion is
        /// satisfied.
        next: StateID,
    },
    /// An alternation such that there exists an epsilon transition to all
    /// states in `alternates`, where matches found via earlier transitions
    /// are preferred over later transitions.
    Union {
        /// An ordered sequence of unconditional epsilon transitions to other
        /// states. Transitions earlier in the sequence are preferred over
        /// transitions later in the sequence.
        alternates: Box<[StateID]>,
    },
    /// An alternation such that there exists precisely two unconditional
    /// epsilon transitions, where matches found via `alt1` are preferred over
    /// matches found via `alt2`.
    ///
    /// This state exists as a common special case of Union where there are
    /// only two alternates. In this case, we don't need any allocations to
    /// represent the state. This saves a bit of memory and also saves an
    /// additional memory access when traversing the NFA.
    BinaryUnion {
        /// An unconditional epsilon transition to another NFA state. This
        /// is preferred over `alt2`.
        alt1: StateID,
        /// An unconditional epsilon transition to another NFA state. Matches
        /// reported via this transition should only be reported if no matches
        /// were found by following `alt1`.
        alt2: StateID,
    },
    /// An empty state that records a capture location.
    ///
    /// From the perspective of finite automata, this is precisely equivalent
    /// to an unconditional epsilon transition, but serves the purpose of
    /// instructing NFA simulations to record additional state when the finite
    /// state machine passes through this epsilon transition.
    ///
    /// `slot` in this context refers to the specific capture group slot
    /// offset that is being recorded. Each capturing group has two slots
    /// corresponding to the start and end of the matching portion of that
    /// group.
    ///
    /// The pattern ID and capture group index are also included in this state
    /// in case they are useful. But mostly, all you'll need is `next` and
    /// `slot`.
    Capture {
        /// The state to transition to, unconditionally.
        next: StateID,
        /// The pattern ID that this capture belongs to.
        pattern_id: PatternID,
        /// The capture group index that this capture belongs to. Capture group
        /// indices are local to each pattern. For example, when capturing
        /// groups are enabled, every pattern has a capture group at index
        /// `0`.
        group_index: SmallIndex,
        /// The slot index for this capture. Every capturing group has two
        /// slots: one for the start haystack offset and one for the end
        /// haystack offset. Unlike capture group indices, slot indices are
        /// global across all patterns in this NFA. That is, each slot belongs
        /// to a single pattern, but there is only one slot at index `i`.
        slot: SmallIndex,
    },
    /// A state that cannot be transitioned out of. This is useful for cases
    /// where you want to prevent matching from occurring. For example, if your
    /// regex parser permits empty character classes, then one could choose
    /// a `Fail` state to represent them. (An empty character class can be
    /// thought of as an empty set. Since nothing is in an empty set, they can
    /// never match anything.)
    Fail,
    /// A match state. There is at least one such occurrence of this state for
    /// each regex that can match that is in this NFA.
    Match {
        /// The matching pattern ID.
        pattern_id: PatternID,
    },
}
impl Visited {
    const BLOCK_SIZE: usize = 8 * core::mem::size_of::<usize>();
    fn new(re: &BoundedBacktracker) -> Visited {}
    fn insert(&mut self, sid: StateID, at: usize) -> bool {}
    fn reset(&mut self, _: &BoundedBacktracker) {}
    fn setup_search(
        &mut self,
        re: &BoundedBacktracker,
        input: &Input<'_>,
    ) -> Result<(), MatchError> {
        let haylen = input.get_span().len();
        let err = || MatchError::haystack_too_long(haylen);
        self.stride = haylen + 1;
        let needed_capacity = match re.get_nfa().states().len().checked_mul(self.stride)
        {
            None => return Err(err()),
            Some(capacity) => capacity,
        };
        let max_capacity = 8 * re.get_config().get_visited_capacity();
        if needed_capacity > max_capacity {
            return Err(err());
        }
        let needed_blocks = div_ceil(needed_capacity, Visited::BLOCK_SIZE);
        self.bitset.truncate(needed_blocks);
        for block in self.bitset.iter_mut() {
            *block = 0;
        }
        if needed_blocks > self.bitset.len() {
            self.bitset.resize(needed_blocks, 0);
        }
        Ok(())
    }
    fn memory_usage(&self) -> usize {}
}
impl BoundedBacktracker {
    #[cfg(feature = "syntax")]
    pub fn new(pattern: &str) -> Result<BoundedBacktracker, BuildError> {}
    #[cfg(feature = "syntax")]
    pub fn new_many<P: AsRef<str>>(
        patterns: &[P],
    ) -> Result<BoundedBacktracker, BuildError> {}
    pub fn new_from_nfa(nfa: NFA) -> Result<BoundedBacktracker, BuildError> {}
    pub fn always_match() -> Result<BoundedBacktracker, BuildError> {}
    pub fn never_match() -> Result<BoundedBacktracker, BuildError> {}
    pub fn config() -> Config {}
    pub fn builder() -> Builder {}
    pub fn create_cache(&self) -> Cache {}
    pub fn create_captures(&self) -> Captures {}
    pub fn reset_cache(&self, cache: &mut Cache) {}
    pub fn pattern_len(&self) -> usize {}
    #[inline]
    pub fn get_config(&self) -> &Config {
        &self.config
    }
    #[inline]
    pub fn get_nfa(&self) -> &NFA {
        &self.nfa
    }
    #[inline]
    pub fn max_haystack_len(&self) -> usize {}
}
impl Config {
    pub fn new() -> Config {}
    pub fn prefilter(mut self, pre: Option<Prefilter>) -> Config {}
    pub fn visited_capacity(mut self, capacity: usize) -> Config {}
    pub fn get_prefilter(&self) -> Option<&Prefilter> {}
    pub fn get_visited_capacity(&self) -> usize {
        const DEFAULT: usize = 256 * (1 << 10);
        self.visited_capacity.unwrap_or(DEFAULT)
    }
    pub(crate) fn overwrite(&self, o: Config) -> Config {}
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
    pub fn haystack(&self) -> &[u8] {}
    #[inline]
    pub fn start(&self) -> usize {}
    #[inline]
    pub fn end(&self) -> usize {}
    #[inline]
    pub fn get_span(&self) -> Span {
        self.span
    }
    #[inline]
    pub fn get_range(&self) -> Range<usize> {}
    #[inline]
    pub fn get_anchored(&self) -> Anchored {}
    #[inline]
    pub fn get_earliest(&self) -> bool {}
    #[inline]
    pub fn is_done(&self) -> bool {}
    #[inline]
    pub fn is_char_boundary(&self, offset: usize) -> bool {}
}
impl Span {
    #[inline]
    pub fn range(&self) -> Range<usize> {}
    #[inline]
    pub fn is_empty(&self) -> bool {}
    #[inline]
    pub fn len(&self) -> usize {
        self.end.saturating_sub(self.start)
    }
    #[inline]
    pub fn contains(&self, offset: usize) -> bool {}
    #[inline]
    pub fn offset(&self, offset: usize) -> Span {}
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
    pub fn states(&self) -> &[State] {
        &self.0.states
    }
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
fn div_ceil(lhs: usize, rhs: usize) -> usize {
    if lhs % rhs == 0 { lhs / rhs } else { (lhs / rhs) + 1 }
}
