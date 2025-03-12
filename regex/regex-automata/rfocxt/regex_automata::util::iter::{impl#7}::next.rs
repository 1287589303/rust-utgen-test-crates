#[cfg(feature = "alloc")]
use crate::util::captures::Captures;
use crate::util::search::{HalfMatch, Input, Match, MatchError};
pub struct TryMatchesIter<'h, F> {
    it: Searcher<'h>,
    finder: F,
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchError(
    #[cfg(feature = "alloc")]
    alloc::boxed::Box<MatchErrorKind>,
    #[cfg(not(feature = "alloc"))]
    MatchErrorKind,
);
impl<'h, F> Iterator for TryMatchesIter<'h, F>
where
    F: FnMut(&Input<'_>) -> Result<Option<Match>, MatchError>,
{
    type Item = Result<Match, MatchError>;
    #[inline]
    fn next(&mut self) -> Option<Result<Match, MatchError>> {
        self.it.try_advance(&mut self.finder).transpose()
    }
}
