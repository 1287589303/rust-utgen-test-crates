#[cfg(feature = "alloc")]
use crate::util::captures::Captures;
use crate::util::search::{HalfMatch, Input, Match, MatchError};
pub struct TryMatchesIter<'h, F> {
    it: Searcher<'h>,
    finder: F,
}
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
impl<'h, F> TryMatchesIter<'h, F> {
    pub fn infallible(self) -> MatchesIter<'h, F> {
        MatchesIter(self)
    }
    pub fn input<'i>(&'i self) -> &'i Input<'h> {}
}
