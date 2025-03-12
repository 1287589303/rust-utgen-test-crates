#[cfg(feature = "alloc")]
use crate::util::search::PatternSet;
use crate::{
    dfa::search,
    util::{
        empty, prefilter::Prefilter, primitives::{PatternID, StateID},
        search::{Anchored, HalfMatch, Input, MatchError},
        start,
    },
};
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
#[derive(Clone, Copy)]
pub struct DebugByte(pub u8);
#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum StartError {
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
impl core::fmt::Display for StartError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            StartError::Quit { byte } => {
                write!(
                    f,
                    "error computing start state because the look-behind byte \
                 {:?} triggered a quit state",
                    crate ::util::escape::DebugByte(byte),
                )
            }
            StartError::UnsupportedAnchored { mode: Anchored::Yes } => {
                write!(
                    f,
                    "error computing start state because \
                     anchored searches are not supported or enabled"
                )
            }
            StartError::UnsupportedAnchored { mode: Anchored::No } => {
                write!(
                    f,
                    "error computing start state because \
                     unanchored searches are not supported or enabled"
                )
            }
            StartError::UnsupportedAnchored { mode: Anchored::Pattern(pid) } => {
                write!(
                    f,
                    "error computing start state because \
                     anchored searches for a specific pattern ({}) \
                     are not supported or enabled",
                    pid.as_usize(),
                )
            }
        }
    }
}
