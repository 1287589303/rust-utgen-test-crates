use crate::{
    dfa::{accel, automaton::{Automaton, OverlappingState}},
    util::{
        prefilter::Prefilter, primitives::StateID,
        search::{Anchored, HalfMatch, Input, Span},
    },
    MatchError,
};
#[derive(Clone)]
pub struct Input<'h> {
    haystack: &'h [u8],
    span: Span,
    anchored: Anchored,
    earliest: bool,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchError(
    #[cfg(feature = "alloc")]
    alloc::boxed::Box<MatchErrorKind>,
    #[cfg(not(feature = "alloc"))]
    MatchErrorKind,
);
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OverlappingState {
    /// The match reported by the most recent overlapping search to use this
    /// state.
    ///
    /// If a search does not find any matches, then it is expected to clear
    /// this value.
    pub(crate) mat: Option<HalfMatch>,
    /// The state ID of the state at which the search was in when the call
    /// terminated. When this is a match state, `last_match` must be set to a
    /// non-None value.
    ///
    /// A `None` value indicates the start state of the corresponding
    /// automaton. We cannot use the actual ID, since any one automaton may
    /// have many start states, and which one is in use depends on several
    /// search-time factors.
    pub(crate) id: Option<StateID>,
    /// The position of the search.
    ///
    /// When `id` is None (i.e., we are starting a search), this is set to
    /// the beginning of the search as given by the caller regardless of its
    /// current value. Subsequent calls to an overlapping search pick up at
    /// this offset.
    pub(crate) at: usize,
    /// The index into the matching patterns of the next match to report if the
    /// current state is a match state. Note that this may be 1 greater than
    /// the total number of matches to report for the current match state. (In
    /// which case, no more matches should be reported at the current position
    /// and the search should advance to the next position.)
    pub(crate) next_match_index: Option<usize>,
    /// This is set to true when a reverse overlapping search has entered its
    /// EOI transitions.
    ///
    /// This isn't used in a forward search because it knows to stop once the
    /// position exceeds the end of the search range. In a reverse search,
    /// since we use unsigned offsets, we don't "know" once we've gone past
    /// `0`. So the only way to detect it is with this extra flag. The reverse
    /// overlapping search knows to terminate specifically after it has
    /// reported all matches after following the EOI transition.
    pub(crate) rev_eoi: bool,
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
    pub fn get_span(&self) -> Span {}
    #[inline]
    pub fn get_range(&self) -> Range<usize> {}
    #[inline]
    pub fn get_anchored(&self) -> Anchored {
        self.anchored
    }
    #[inline]
    pub fn get_earliest(&self) -> bool {}
    #[inline]
    pub fn is_done(&self) -> bool {
        self.get_span().start > self.get_span().end
    }
    #[inline]
    pub fn is_char_boundary(&self, offset: usize) -> bool {}
}
impl Anchored {
    #[inline]
    pub fn is_anchored(&self) -> bool {
        matches!(* self, Anchored::Yes | Anchored::Pattern(_))
    }
    #[inline]
    pub fn pattern(&self) -> Option<PatternID> {}
}
#[inline(never)]
pub fn find_overlapping_fwd<A: Automaton + ?Sized>(
    dfa: &A,
    input: &Input<'_>,
    state: &mut OverlappingState,
) -> Result<(), MatchError> {
    state.mat = None;
    if input.is_done() {
        return Ok(());
    }
    let pre = if input.get_anchored().is_anchored() {
        None
    } else {
        dfa.get_prefilter()
    };
    if pre.is_some() {
        find_overlapping_fwd_imp(dfa, input, pre, state)
    } else {
        find_overlapping_fwd_imp(dfa, input, None, state)
    }
}
#[cfg_attr(feature = "perf-inline", inline(always))]
fn find_overlapping_fwd_imp<A: Automaton + ?Sized>(
    dfa: &A,
    input: &Input<'_>,
    pre: Option<&'_ Prefilter>,
    state: &mut OverlappingState,
) -> Result<(), MatchError> {
    let universal_start = dfa.universal_start_state(Anchored::No).is_some();
    let mut sid = match state.id {
        None => {
            state.at = input.start();
            init_fwd(dfa, input)?
        }
        Some(sid) => {
            if let Some(match_index) = state.next_match_index {
                let match_len = dfa.match_len(sid);
                if match_index < match_len {
                    state.next_match_index = Some(match_index + 1);
                    let pattern = dfa.match_pattern(sid, match_index);
                    state.mat = Some(HalfMatch::new(pattern, state.at));
                    return Ok(());
                }
            }
            state.at += 1;
            if state.at > input.end() {
                return Ok(());
            }
            sid
        }
    };
    while state.at < input.end() {
        sid = dfa.next_state(sid, input.haystack()[state.at]);
        if dfa.is_special_state(sid) {
            state.id = Some(sid);
            if dfa.is_start_state(sid) {
                if let Some(ref pre) = pre {
                    let span = Span::from(state.at..input.end());
                    match pre.find(input.haystack(), span) {
                        None => return Ok(()),
                        Some(ref span) => {
                            if span.start > state.at {
                                state.at = span.start;
                                if !universal_start {
                                    sid = prefilter_restart(dfa, &input, state.at)?;
                                }
                                continue;
                            }
                        }
                    }
                } else if dfa.is_accel_state(sid) {
                    let needles = dfa.accelerator(sid);
                    state.at = accel::find_fwd(needles, input.haystack(), state.at + 1)
                        .unwrap_or(input.end());
                    continue;
                }
            } else if dfa.is_match_state(sid) {
                state.next_match_index = Some(1);
                let pattern = dfa.match_pattern(sid, 0);
                state.mat = Some(HalfMatch::new(pattern, state.at));
                return Ok(());
            } else if dfa.is_accel_state(sid) {
                let needs = dfa.accelerator(sid);
                state.at = accel::find_fwd(needs, input.haystack(), state.at + 1)
                    .unwrap_or(input.end());
                continue;
            } else if dfa.is_dead_state(sid) {
                return Ok(());
            } else {
                return Err(MatchError::quit(input.haystack()[state.at], state.at));
            }
        }
        state.at += 1;
    }
    let result = eoi_fwd(dfa, input, &mut sid, &mut state.mat);
    state.id = Some(sid);
    if state.mat.is_some() {
        state.next_match_index = Some(1);
    }
    result
}
