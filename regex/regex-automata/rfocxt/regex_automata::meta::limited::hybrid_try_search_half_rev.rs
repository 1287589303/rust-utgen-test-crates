use crate::{
    meta::error::{RetryError, RetryQuadraticError},
    HalfMatch, Input, MatchError,
};
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub struct LazyStateID(u32);
#[derive(Clone, Debug)]
pub struct DFA {
    config: Config,
    nfa: thompson::NFA,
    stride2: usize,
    start_map: StartByteMap,
    classes: ByteClasses,
    quitset: ByteSet,
    cache_capacity: usize,
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
#[derive(Clone)]
pub struct Input<'h> {
    haystack: &'h [u8],
    span: Span,
    anchored: Anchored,
    earliest: bool,
}
#[derive(Debug)]
pub(crate) struct RetryQuadraticError(());
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchError(
    #[cfg(feature = "alloc")]
    alloc::boxed::Box<MatchErrorKind>,
    #[cfg(not(feature = "alloc"))]
    MatchErrorKind,
);
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
#[derive(Clone, Debug)]
pub struct CacheError(());
#[derive(Clone, Debug)]
pub struct Cache {
    /// The transition table.
    ///
    /// Given a `current` LazyStateID and an `input` byte, the next state can
    /// be computed via `trans[untagged(current) + equiv_class(input)]`. Notice
    /// that no multiplication is used. That's because state identifiers are
    /// "premultiplied."
    ///
    /// Note that the next state may be the "unknown" state. In this case, the
    /// next state is not known and determinization for `current` on `input`
    /// must be performed.
    trans: Vec<LazyStateID>,
    /// The starting states for this DFA.
    ///
    /// These are computed lazily. Initially, these are all set to "unknown"
    /// lazy state IDs.
    ///
    /// When 'starts_for_each_pattern' is disabled (the default), then the size
    /// of this is constrained to the possible starting configurations based
    /// on the search parameters. (At time of writing, that's 4.) However,
    /// when starting states for each pattern is enabled, then there are N
    /// additional groups of starting states, where each group reflects the
    /// different possible configurations and N is the number of patterns.
    starts: Vec<LazyStateID>,
    /// A sequence of NFA/DFA powerset states that have been computed for this
    /// lazy DFA. This sequence is indexable by untagged LazyStateIDs. (Every
    /// tagged LazyStateID can be used to index this sequence by converting it
    /// to its untagged form.)
    states: Vec<State>,
    /// A map from states to their corresponding IDs. This map may be accessed
    /// via the raw byte representation of a state, which means that a `State`
    /// does not need to be allocated to determine whether it already exists
    /// in this map. Indeed, the existence of such a state is what determines
    /// whether we allocate a new `State` or not.
    ///
    /// The higher level idea here is that we do just enough determinization
    /// for a state to check whether we've already computed it. If we have,
    /// then we can save a little (albeit not much) work. The real savings is
    /// in memory usage. If we never checked for trivially duplicate states,
    /// then our memory usage would explode to unreasonable levels.
    states_to_id: StateMap,
    /// Sparse sets used to track which NFA states have been visited during
    /// various traversals.
    sparses: SparseSets,
    /// Scratch space for traversing the NFA graph. (We use space on the heap
    /// instead of the call stack.)
    stack: Vec<NFAStateID>,
    /// Scratch space for building a NFA/DFA powerset state. This is used to
    /// help amortize allocation since not every powerset state generated is
    /// added to the cache. In particular, if it already exists in the cache,
    /// then there is no need to allocate a new `State` for it.
    scratch_state_builder: StateBuilderEmpty,
    /// A simple abstraction for handling the saving of at most a single state
    /// across a cache clearing. This is required for correctness. Namely, if
    /// adding a new state after clearing the cache fails, then the caller
    /// must retain the ability to continue using the state ID given. The
    /// state corresponding to the state ID is what we preserve across cache
    /// clearings.
    state_saver: StateSaver,
    /// The memory usage, in bytes, used by 'states' and 'states_to_id'. We
    /// track this as new states are added since states use a variable amount
    /// of heap. Tracking this as we add states makes it possible to compute
    /// the total amount of memory used by the determinizer in constant time.
    memory_usage_state: usize,
    /// The number of times the cache has been cleared. When a minimum cache
    /// clear count is set, then the cache will return an error instead of
    /// clearing the cache if the count has been exceeded.
    clear_count: usize,
    /// The total number of bytes searched since the last time this cache was
    /// cleared, not including the current search.
    ///
    /// This can be added to the length of the current search to get the true
    /// total number of bytes searched.
    ///
    /// This is generally only non-zero when the
    /// `Cache::search_{start,update,finish}` APIs are used to track search
    /// progress.
    bytes_searched: usize,
    /// The progress of the current search.
    ///
    /// This is only non-`None` when callers utlize the `Cache::search_start`,
    /// `Cache::search_update` and `Cache::search_finish` APIs.
    ///
    /// The purpose of recording search progress is to be able to make a
    /// determination about the efficiency of the cache. Namely, by keeping
    /// track of the
    progress: Option<SearchProgress>,
}
#[derive(Debug)]
pub(crate) enum RetryError {
    Quadratic(RetryQuadraticError),
    Fail(RetryFailError),
}
impl LazyStateID {
    #[cfg(any(target_pointer_width = "32", target_pointer_width = "64"))]
    const MAX_BIT: usize = 31;
    #[cfg(target_pointer_width = "16")]
    const MAX_BIT: usize = 15;
    const MASK_UNKNOWN: usize = 1 << (LazyStateID::MAX_BIT);
    const MASK_DEAD: usize = 1 << (LazyStateID::MAX_BIT - 1);
    const MASK_QUIT: usize = 1 << (LazyStateID::MAX_BIT - 2);
    const MASK_START: usize = 1 << (LazyStateID::MAX_BIT - 3);
    const MASK_MATCH: usize = 1 << (LazyStateID::MAX_BIT - 4);
    const MAX: usize = LazyStateID::MASK_MATCH - 1;
    #[inline]
    pub(crate) fn new(id: usize) -> Result<LazyStateID, LazyStateIDError> {}
    #[inline]
    const fn new_unchecked(id: usize) -> LazyStateID {}
    #[inline]
    pub(crate) fn as_usize_untagged(&self) -> usize {}
    #[inline]
    pub(crate) const fn as_usize_unchecked(&self) -> usize {}
    #[inline]
    pub(crate) const fn to_unknown(&self) -> LazyStateID {}
    #[inline]
    pub(crate) const fn to_dead(&self) -> LazyStateID {}
    #[inline]
    pub(crate) const fn to_quit(&self) -> LazyStateID {}
    #[inline]
    pub(crate) const fn to_start(&self) -> LazyStateID {}
    #[inline]
    pub(crate) const fn to_match(&self) -> LazyStateID {}
    #[inline]
    pub const fn is_tagged(&self) -> bool {
        self.as_usize_unchecked() > LazyStateID::MAX
    }
    #[inline]
    pub const fn is_unknown(&self) -> bool {}
    #[inline]
    pub const fn is_dead(&self) -> bool {
        self.as_usize_unchecked() & LazyStateID::MASK_DEAD > 0
    }
    #[inline]
    pub const fn is_quit(&self) -> bool {
        self.as_usize_unchecked() & LazyStateID::MASK_QUIT > 0
    }
    #[inline]
    pub const fn is_start(&self) -> bool {}
    #[inline]
    pub const fn is_match(&self) -> bool {
        self.as_usize_unchecked() & LazyStateID::MASK_MATCH > 0
    }
}
impl DFA {
    #[inline]
    pub fn next_state(
        &self,
        cache: &mut Cache,
        current: LazyStateID,
        input: u8,
    ) -> Result<LazyStateID, CacheError> {
        let class = usize::from(self.classes.get(input));
        let offset = current.as_usize_untagged() + class;
        let sid = cache.trans[offset];
        if !sid.is_unknown() {
            return Ok(sid);
        }
        let unit = alphabet::Unit::u8(input);
        Lazy::new(self, cache).cache_next_state(current, unit)
    }
    #[inline]
    pub fn next_state_untagged(
        &self,
        cache: &Cache,
        current: LazyStateID,
        input: u8,
    ) -> LazyStateID {}
    #[inline]
    pub unsafe fn next_state_untagged_unchecked(
        &self,
        cache: &Cache,
        current: LazyStateID,
        input: u8,
    ) -> LazyStateID {}
    #[inline]
    pub fn next_eoi_state(
        &self,
        cache: &mut Cache,
        current: LazyStateID,
    ) -> Result<LazyStateID, CacheError> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub fn start_state(
        &self,
        cache: &mut Cache,
        config: &start::Config,
    ) -> Result<LazyStateID, StartError> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub fn start_state_forward(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
    ) -> Result<LazyStateID, MatchError> {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub fn start_state_reverse(
        &self,
        cache: &mut Cache,
        input: &Input<'_>,
    ) -> Result<LazyStateID, MatchError> {
        let config = start::Config::from_input_reverse(input);
        self.start_state(cache, &config)
            .map_err(|err| match err {
                StartError::Cache { .. } => MatchError::gave_up(input.end()),
                StartError::Quit { byte } => {
                    let offset = input.end();
                    MatchError::quit(byte, offset)
                }
                StartError::UnsupportedAnchored { mode } => {
                    MatchError::unsupported_anchored(mode)
                }
            })
    }
    #[inline]
    pub fn match_len(&self, cache: &Cache, id: LazyStateID) -> usize {}
    #[inline]
    pub fn match_pattern(
        &self,
        cache: &Cache,
        id: LazyStateID,
        match_index: usize,
    ) -> PatternID {
        if self.pattern_len() == 1 {
            return PatternID::ZERO;
        }
        LazyRef::new(self, cache).get_cached_state(id).match_pattern(match_index)
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
    pub fn get_earliest(&self) -> bool {}
    #[inline]
    pub fn is_done(&self) -> bool {}
    #[inline]
    pub fn is_char_boundary(&self, offset: usize) -> bool {}
}
impl RetryQuadraticError {
    pub(crate) fn new() -> RetryQuadraticError {
        RetryQuadraticError(())
    }
}
impl MatchError {
    pub fn new(kind: MatchErrorKind) -> MatchError {}
    pub fn kind(&self) -> &MatchErrorKind {}
    pub fn quit(byte: u8, offset: usize) -> MatchError {
        MatchError::new(MatchErrorKind::Quit {
            byte,
            offset,
        })
    }
    pub fn gave_up(offset: usize) -> MatchError {}
    pub fn haystack_too_long(len: usize) -> MatchError {}
    pub fn unsupported_anchored(mode: Anchored) -> MatchError {}
}
#[cfg(feature = "hybrid")]
pub(crate) fn hybrid_try_search_half_rev(
    dfa: &crate::hybrid::dfa::DFA,
    cache: &mut crate::hybrid::dfa::Cache,
    input: &Input<'_>,
    min_start: usize,
) -> Result<Option<HalfMatch>, RetryError> {
    let mut mat = None;
    let mut sid = dfa.start_state_reverse(cache, input)?;
    if input.start() == input.end() {
        hybrid_eoi_rev(dfa, cache, input, &mut sid, &mut mat)?;
        return Ok(mat);
    }
    let mut at = input.end() - 1;
    loop {
        sid = dfa
            .next_state(cache, sid, input.haystack()[at])
            .map_err(|_| MatchError::gave_up(at))?;
        if sid.is_tagged() {
            if sid.is_match() {
                let pattern = dfa.match_pattern(cache, sid, 0);
                mat = Some(HalfMatch::new(pattern, at + 1));
            } else if sid.is_dead() {
                return Ok(mat);
            } else if sid.is_quit() {
                return Err(MatchError::quit(input.haystack()[at], at).into());
            }
        }
        if at == input.start() {
            break;
        }
        at -= 1;
        if at < min_start {
            trace!(
                "reached position {} which is before the previous literal \
				 match, quitting to avoid quadratic behavior",
                at,
            );
            return Err(RetryError::Quadratic(RetryQuadraticError::new()));
        }
    }
    let was_dead = sid.is_dead();
    hybrid_eoi_rev(dfa, cache, input, &mut sid, &mut mat)?;
    if at == input.start() && mat.map_or(false, |m| m.offset() > input.start())
        && !was_dead
    {
        trace!(
            "reached beginning of search at offset {} without hitting \
             a dead state, quitting to avoid potential false positive match",
            at,
        );
        return Err(RetryError::Quadratic(RetryQuadraticError::new()));
    }
    Ok(mat)
}
#[cfg(feature = "hybrid")]
#[cfg_attr(feature = "perf-inline", inline(always))]
fn hybrid_eoi_rev(
    dfa: &crate::hybrid::dfa::DFA,
    cache: &mut crate::hybrid::dfa::Cache,
    input: &Input<'_>,
    sid: &mut crate::hybrid::LazyStateID,
    mat: &mut Option<HalfMatch>,
) -> Result<(), MatchError> {
    let sp = input.get_span();
    if sp.start > 0 {
        let byte = input.haystack()[sp.start - 1];
        *sid = dfa
            .next_state(cache, *sid, byte)
            .map_err(|_| MatchError::gave_up(sp.start))?;
        if sid.is_match() {
            let pattern = dfa.match_pattern(cache, *sid, 0);
            *mat = Some(HalfMatch::new(pattern, sp.start));
        } else if sid.is_quit() {
            return Err(MatchError::quit(byte, sp.start - 1));
        }
    } else {
        *sid = dfa
            .next_eoi_state(cache, *sid)
            .map_err(|_| MatchError::gave_up(sp.start))?;
        if sid.is_match() {
            let pattern = dfa.match_pattern(cache, *sid, 0);
            *mat = Some(HalfMatch::new(pattern, 0));
        }
        debug_assert!(! sid.is_quit());
    }
    Ok(())
}
