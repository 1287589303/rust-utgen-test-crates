use regex_syntax::{ast, hir};
use crate::{nfa, util::search::MatchError, PatternID};
#[derive(Debug)]
pub(crate) struct RetryFailError {
    offset: usize,
}
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MatchError(
    #[cfg(feature = "alloc")]
    alloc::boxed::Box<MatchErrorKind>,
    #[cfg(not(feature = "alloc"))]
    MatchErrorKind,
);
#[non_exhaustive]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MatchErrorKind {
    /// The search saw a "quit" byte at which it was instructed to stop
    /// searching.
    Quit {
        /// The "quit" byte that was observed that caused the search to stop.
        byte: u8,
        /// The offset at which the quit byte was observed.
        offset: usize,
    },
    /// The search, based on heuristics, determined that it would be better
    /// to stop, typically to provide the caller an opportunity to use an
    /// alternative regex engine.
    ///
    /// Currently, the only way for this to occur is via the lazy DFA and
    /// only when it is configured to do so (it will not return this error by
    /// default).
    GaveUp {
        /// The offset at which the search stopped. This corresponds to the
        /// position immediately following the last byte scanned.
        offset: usize,
    },
    /// This error occurs if the haystack given to the regex engine was too
    /// long to be searched. This occurs, for example, with regex engines
    /// like the bounded backtracker that have a configurable fixed amount of
    /// capacity that is tied to the length of the haystack. Anything beyond
    /// that configured limit will result in an error at search time.
    HaystackTooLong {
        /// The length of the haystack that exceeded the limit.
        len: usize,
    },
    /// An error indicating that a particular type of anchored search was
    /// requested, but that the regex engine does not support it.
    ///
    /// Note that this error should not be returned by a regex engine simply
    /// because the pattern ID is invalid (i.e., equal to or exceeds the number
    /// of patterns in the regex). In that case, the regex engine should report
    /// a non-match.
    UnsupportedAnchored {
        /// The anchored mode given that is unsupported.
        mode: Anchored,
    },
}
impl From<MatchError> for RetryFailError {
    fn from(merr: MatchError) -> RetryFailError {
        use crate::util::search::MatchErrorKind::*;
        match *merr.kind() {
            Quit { offset, .. } => RetryFailError::from_offset(offset),
            GaveUp { offset } => RetryFailError::from_offset(offset),
            HaystackTooLong { .. } | UnsupportedAnchored { .. } => {
                unreachable!("found impossible error in meta engine: {}", merr)
            }
        }
    }
}
impl MatchError {
    pub fn new(kind: MatchErrorKind) -> MatchError {}
    pub fn kind(&self) -> &MatchErrorKind {
        &self.0
    }
    pub fn quit(byte: u8, offset: usize) -> MatchError {}
    pub fn gave_up(offset: usize) -> MatchError {}
    pub fn haystack_too_long(len: usize) -> MatchError {}
    pub fn unsupported_anchored(mode: Anchored) -> MatchError {}
}
impl RetryFailError {
    pub(crate) fn from_offset(offset: usize) -> RetryFailError {
        RetryFailError { offset }
    }
}
