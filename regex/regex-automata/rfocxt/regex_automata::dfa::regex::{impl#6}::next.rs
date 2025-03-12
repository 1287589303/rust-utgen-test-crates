#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[cfg(feature = "dfa-build")]
use crate::dfa::dense::BuildError;
use crate::{
    dfa::{automaton::Automaton, dense},
    util::{iter, search::Input},
    Anchored, Match, MatchError,
};
#[cfg(feature = "alloc")]
use crate::{
    dfa::{sparse, StartKind},
    util::search::MatchKind,
};
pub unsafe trait Automaton {
    fn next_state(&self, current: StateID, input: u8) -> StateID;
    unsafe fn next_state_unchecked(&self, current: StateID, input: u8) -> StateID;
    fn next_eoi_state(&self, current: StateID) -> StateID;
    fn start_state(&self, config: &start::Config) -> Result<StateID, StartError>;
    fn start_state_forward(&self, input: &Input<'_>) -> Result<StateID, MatchError>;
    fn start_state_reverse(&self, input: &Input<'_>) -> Result<StateID, MatchError>;
    #[inline]
    fn universal_start_state(&self, _mode: Anchored) -> Option<StateID>;
    fn is_special_state(&self, id: StateID) -> bool;
    fn is_dead_state(&self, id: StateID) -> bool;
    fn is_quit_state(&self, id: StateID) -> bool;
    fn is_match_state(&self, id: StateID) -> bool;
    fn is_start_state(&self, id: StateID) -> bool;
    fn is_accel_state(&self, id: StateID) -> bool;
    fn pattern_len(&self) -> usize;
    fn match_len(&self, id: StateID) -> usize;
    fn match_pattern(&self, id: StateID, index: usize) -> PatternID;
    fn has_empty(&self) -> bool;
    fn is_utf8(&self) -> bool;
    fn is_always_start_anchored(&self) -> bool;
    #[inline]
    fn accelerator(&self, _id: StateID) -> &[u8];
    #[inline]
    fn get_prefilter(&self) -> Option<&Prefilter>;
    #[inline]
    fn try_search_fwd(&self, input: &Input<'_>) -> Result<Option<HalfMatch>, MatchError>;
    #[inline]
    fn try_search_rev(&self, input: &Input<'_>) -> Result<Option<HalfMatch>, MatchError>;
    #[inline]
    fn try_search_overlapping_fwd(
        &self,
        input: &Input<'_>,
        state: &mut OverlappingState,
    ) -> Result<(), MatchError>;
    #[inline]
    fn try_search_overlapping_rev(
        &self,
        input: &Input<'_>,
        state: &mut OverlappingState,
    ) -> Result<(), MatchError>;
    #[cfg(feature = "alloc")]
    #[inline]
    fn try_which_overlapping_matches(
        &self,
        input: &Input<'_>,
        patset: &mut PatternSet,
    ) -> Result<(), MatchError>;
}
#[derive(Debug)]
pub struct FindMatches<'r, 'h, A> {
    re: &'r Regex<A>,
    it: iter::Searcher<'h>,
}
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Match {
    /// The pattern ID.
    pattern: PatternID,
    /// The underlying match span.
    span: Span,
}
#[derive(Clone, Debug)]
pub struct Searcher<'h> {
    /// The input parameters to give to each regex engine call.
    ///
    /// The start position of the search is mutated during iteration.
    input: Input<'h>,
    /// Records the end offset of the most recent match. This is necessary to
    /// handle a corner case for preventing empty matches from overlapping with
    /// the ending bounds of a prior match.
    last_match_end: Option<usize>,
}
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
#[derive(Debug)]
pub struct Regex {
    /// The forward lazy DFA. This can only find the end of a match.
    forward: DFA,
    /// The reverse lazy DFA. This can only find the start of a match.
    ///
    /// This is built with 'all' match semantics (instead of leftmost-first)
    /// so that it always finds the longest possible match (which corresponds
    /// to the leftmost starting position). It is also compiled as an anchored
    /// matcher and has 'starts_for_each_pattern' enabled. Including starting
    /// states for each pattern is necessary to ensure that we only look for
    /// matches of a pattern that matched in the forward direction. Otherwise,
    /// we might wind up finding the "leftmost" starting position of a totally
    /// different pattern!
    reverse: DFA,
}
impl<'r, 'h, A: Automaton> Iterator for FindMatches<'r, 'h, A> {
    type Item = Match;
    #[inline]
    fn next(&mut self) -> Option<Match> {
        let FindMatches { re, ref mut it } = *self;
        it.advance(|input| re.try_search(input))
    }
}
