#[cfg(feature = "alloc")]
use crate::util::captures::Captures;
use crate::util::search::{HalfMatch, Input, Match, MatchError};
#[derive(Debug)]
pub struct HalfMatchesIter<'h, F>(TryHalfMatchesIter<'h, F>);
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchError(
    #[cfg(feature = "alloc")]
    alloc::boxed::Box<MatchErrorKind>,
    #[cfg(not(feature = "alloc"))]
    MatchErrorKind,
);
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
pub struct TryHalfMatchesIter<'h, F> {
    it: Searcher<'h>,
    finder: F,
}
impl<'h, F> Iterator for HalfMatchesIter<'h, F>
where
    F: FnMut(&Input<'_>) -> Result<Option<HalfMatch>, MatchError>,
{
    type Item = HalfMatch;
    #[inline]
    fn next(&mut self) -> Option<HalfMatch> {
        match self.0.next()? {
            Ok(m) => Some(m),
            Err(err) => {
                panic!(
                    "unexpected regex half find error: {}\n\
                 to handle find errors, use 'try' or 'search' methods",
                    err,
                )
            }
        }
    }
}
