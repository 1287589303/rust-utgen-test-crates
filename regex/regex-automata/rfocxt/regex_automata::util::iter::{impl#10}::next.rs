#[cfg(feature = "alloc")]
use crate::util::captures::Captures;
use crate::util::search::{HalfMatch, Input, Match, MatchError};
#[derive(Debug)]
pub struct MatchesIter<'h, F>(TryMatchesIter<'h, F>);
pub struct TryMatchesIter<'h, F> {
    it: Searcher<'h>,
    finder: F,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchError(
    #[cfg(feature = "alloc")]
    alloc::boxed::Box<MatchErrorKind>,
    #[cfg(not(feature = "alloc"))]
    MatchErrorKind,
);
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Match {
    /// The pattern ID.
    pattern: PatternID,
    /// The underlying match span.
    span: Span,
}
impl<'h, F> Iterator for MatchesIter<'h, F>
where
    F: FnMut(&Input<'_>) -> Result<Option<Match>, MatchError>,
{
    type Item = Match;
    #[inline]
    fn next(&mut self) -> Option<Match> {
        match self.0.next()? {
            Ok(m) => Some(m),
            Err(err) => {
                panic!(
                    "unexpected regex find error: {}\n\
                 to handle find errors, use 'try' or 'search' methods",
                    err,
                )
            }
        }
    }
}
