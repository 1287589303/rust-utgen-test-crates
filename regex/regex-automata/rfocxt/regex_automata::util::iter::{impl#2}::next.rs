#[cfg(feature = "alloc")]
use crate::util::captures::Captures;
use crate::util::search::{HalfMatch, Input, Match, MatchError};
pub struct TryHalfMatchesIter<'h, F> {
    it: Searcher<'h>,
    finder: F,
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchError(
    #[cfg(feature = "alloc")]
    alloc::boxed::Box<MatchErrorKind>,
    #[cfg(not(feature = "alloc"))]
    MatchErrorKind,
);
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
impl<'h, F> Iterator for TryHalfMatchesIter<'h, F>
where
    F: FnMut(&Input<'_>) -> Result<Option<HalfMatch>, MatchError>,
{
    type Item = Result<HalfMatch, MatchError>;
    #[inline]
    fn next(&mut self) -> Option<Result<HalfMatch, MatchError>> {
        self.it.try_advance_half(&mut self.finder).transpose()
    }
}
