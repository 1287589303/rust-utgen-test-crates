#[cfg(feature = "alloc")]
use crate::util::captures::Captures;
use crate::util::search::{HalfMatch, Input, Match, MatchError};
#[derive(Debug)]
pub struct MatchesIter<'h, F>(TryMatchesIter<'h, F>);
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
pub struct TryMatchesIter<'h, F> {
    it: Searcher<'h>,
    finder: F,
}
#[derive(Clone)]
pub struct Input<'h> {
    haystack: &'h [u8],
    span: Span,
    anchored: Anchored,
    earliest: bool,
}
impl<'h, F> MatchesIter<'h, F> {
    pub fn input<'i>(&'i self) -> &'i Input<'h> {
        self.0.it.input()
    }
}
impl<'h> Searcher<'h> {
    pub fn new(input: Input<'h>) -> Searcher<'h> {}
    pub fn input<'s>(&'s self) -> &'s Input<'h> {
        &self.input
    }
    #[inline]
    pub fn advance_half<F>(&mut self, finder: F) -> Option<HalfMatch>
    where
        F: FnMut(&Input<'_>) -> Result<Option<HalfMatch>, MatchError>,
    {}
    #[inline]
    pub fn advance<F>(&mut self, finder: F) -> Option<Match>
    where
        F: FnMut(&Input<'_>) -> Result<Option<Match>, MatchError>,
    {}
    #[inline]
    pub fn try_advance_half<F>(
        &mut self,
        mut finder: F,
    ) -> Result<Option<HalfMatch>, MatchError>
    where
        F: FnMut(&Input<'_>) -> Result<Option<HalfMatch>, MatchError>,
    {}
    #[inline]
    pub fn try_advance<F>(&mut self, mut finder: F) -> Result<Option<Match>, MatchError>
    where
        F: FnMut(&Input<'_>) -> Result<Option<Match>, MatchError>,
    {}
    #[inline]
    pub fn into_half_matches_iter<F>(self, finder: F) -> TryHalfMatchesIter<'h, F>
    where
        F: FnMut(&Input<'_>) -> Result<Option<HalfMatch>, MatchError>,
    {}
    #[inline]
    pub fn into_matches_iter<F>(self, finder: F) -> TryMatchesIter<'h, F>
    where
        F: FnMut(&Input<'_>) -> Result<Option<Match>, MatchError>,
    {}
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn into_captures_iter<F>(
        self,
        caps: Captures,
        finder: F,
    ) -> TryCapturesIter<'h, F>
    where
        F: FnMut(&Input<'_>, &mut Captures) -> Result<(), MatchError>,
    {}
    #[cold]
    #[inline(never)]
    fn handle_overlapping_empty_half_match<F>(
        &mut self,
        _: HalfMatch,
        mut finder: F,
    ) -> Result<Option<HalfMatch>, MatchError>
    where
        F: FnMut(&Input<'_>) -> Result<Option<HalfMatch>, MatchError>,
    {}
    #[cold]
    #[inline(never)]
    fn handle_overlapping_empty_match<F>(
        &mut self,
        m: Match,
        mut finder: F,
    ) -> Result<Option<Match>, MatchError>
    where
        F: FnMut(&Input<'_>) -> Result<Option<Match>, MatchError>,
    {}
}
