use crate::{hybrid::id::LazyStateIDError, nfa, util::search::Anchored};
#[derive(Clone, Debug)]
pub struct CacheError(());
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum StartError {
    /// An error that occurs when cache inefficiency has dropped below the
    /// configured heuristic thresholds.
    Cache {
        /// The underlying cache error that occurred.
        err: CacheError,
    },
    /// An error that occurs when a starting configuration's look-behind byte
    /// is in this DFA's quit set.
    Quit {
        /// The quit byte that was found.
        byte: u8,
    },
    /// An error that occurs when the caller requests an anchored mode that
    /// isn't supported by the DFA.
    UnsupportedAnchored {
        /// The anchored mode given that is unsupported.
        mode: Anchored,
    },
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
#[cfg(feature = "std")]
impl std::error::Error for StartError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            StartError::Cache { ref err } => Some(err),
            _ => None,
        }
    }
}
