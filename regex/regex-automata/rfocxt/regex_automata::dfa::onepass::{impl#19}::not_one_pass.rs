use alloc::{vec, vec::Vec};
use crate::{
    dfa::{remapper::Remapper, DEAD},
    nfa::thompson::{self, NFA},
    util::{
        alphabet::ByteClasses, captures::Captures, escape::DebugByte,
        int::{Usize, U32, U64, U8},
        look::{Look, LookSet, UnicodeWordBoundaryError},
        primitives::{NonMaxUsize, PatternID, StateID},
        search::{Anchored, Input, Match, MatchError, MatchKind, Span},
        sparse_set::SparseSet,
    },
};
#[derive(Clone, Debug)]
pub struct BuildError {
    kind: BuildErrorKind,
}
#[derive(Clone, Debug)]
enum BuildErrorKind {
    NFA(crate::nfa::thompson::BuildError),
    Word(UnicodeWordBoundaryError),
    TooManyStates { limit: u64 },
    TooManyPatterns { limit: u64 },
    UnsupportedLook { look: Look },
    ExceededSizeLimit { limit: usize },
    NotOnePass { msg: &'static str },
}
#[derive(Clone, Debug)]
enum BuildErrorKind {
    NFA(nfa::thompson::BuildError),
    InsufficientCacheCapacity { minimum: usize, given: usize },
    InsufficientStateIDCapacity { err: LazyStateIDError },
    Unsupported(&'static str),
}
#[derive(Clone, Debug)]
enum BuildErrorKind {
    /// An error that occurred while parsing a regular expression. Note that
    /// this error may be printed over multiple lines, and is generally
    /// intended to be end user readable on its own.
    #[cfg(feature = "syntax")]
    Syntax(regex_syntax::Error),
    /// An error that occurs if the capturing groups provided to an NFA builder
    /// do not satisfy the documented invariants. For example, things like
    /// too many groups, missing groups, having the first (zeroth) group be
    /// named or duplicate group names within the same pattern.
    Captures(captures::GroupInfoError),
    /// An error that occurs when an NFA contains a Unicode word boundary, but
    /// where the crate was compiled without the necessary data for dealing
    /// with Unicode word boundaries.
    Word(look::UnicodeWordBoundaryError),
    /// An error that occurs if too many patterns were given to the NFA
    /// compiler.
    TooManyPatterns {
        /// The number of patterns given, which exceeds the limit.
        given: usize,
        /// The limit on the number of patterns.
        limit: usize,
    },
    /// An error that occurs if too states are produced while building an NFA.
    TooManyStates {
        /// The minimum number of states that are desired, which exceeds the
        /// limit.
        given: usize,
        /// The limit on the number of states.
        limit: usize,
    },
    /// An error that occurs when NFA compilation exceeds a configured heap
    /// limit.
    ExceededSizeLimit {
        /// The configured limit, in bytes.
        limit: usize,
    },
    /// An error that occurs when an invalid capture group index is added to
    /// the NFA. An "invalid" index can be one that would otherwise overflow
    /// a `usize` on the current target.
    InvalidCaptureIndex {
        /// The invalid index that was given.
        index: u32,
    },
    /// An error that occurs when one tries to build a reverse NFA with
    /// captures enabled. Currently, this isn't supported, but we probably
    /// should support it at some point.
    #[cfg(feature = "syntax")]
    UnsupportedCaptures,
}
#[cfg(feature = "dfa-build")]
#[derive(Clone, Debug)]
enum BuildErrorKind {
    /// An error that occurred while constructing an NFA as a precursor step
    /// before a DFA is compiled.
    NFA(thompson::BuildError),
    /// An error that occurred because an unsupported regex feature was used.
    /// The message string describes which unsupported feature was used.
    ///
    /// The primary regex feature that is unsupported by DFAs is the Unicode
    /// word boundary look-around assertion (`\b`). This can be worked around
    /// by either using an ASCII word boundary (`(?-u:\b)`) or by enabling
    /// Unicode word boundaries when building a DFA.
    Unsupported(&'static str),
    /// An error that occurs if too many states are produced while building a
    /// DFA.
    TooManyStates,
    /// An error that occurs if too many start states are needed while building
    /// a DFA.
    ///
    /// This is a kind of oddball error that occurs when building a DFA with
    /// start states enabled for each pattern and enough patterns to cause
    /// the table of start states to overflow `usize`.
    TooManyStartStates,
    /// This is another oddball error that can occur if there are too many
    /// patterns spread out across too many match states.
    TooManyMatchPatternIDs,
    /// An error that occurs if the DFA got too big during determinization.
    DFAExceededSizeLimit { limit: usize },
    /// An error that occurs if auxiliary storage (not the DFA) used during
    /// determinization got too big.
    DeterminizeExceededSizeLimit { limit: usize },
}
#[derive(Clone, Debug)]
enum BuildErrorKind {
    Syntax { pid: PatternID, err: regex_syntax::Error },
    NFA(nfa::thompson::BuildError),
}
impl BuildError {
    fn nfa(err: crate::nfa::thompson::BuildError) -> BuildError {}
    fn word(err: UnicodeWordBoundaryError) -> BuildError {}
    fn too_many_states(limit: u64) -> BuildError {}
    fn too_many_patterns(limit: u64) -> BuildError {}
    fn unsupported_look(look: Look) -> BuildError {}
    fn exceeded_size_limit(limit: usize) -> BuildError {}
    fn not_one_pass(msg: &'static str) -> BuildError {
        BuildError {
            kind: BuildErrorKind::NotOnePass { msg },
        }
    }
}
