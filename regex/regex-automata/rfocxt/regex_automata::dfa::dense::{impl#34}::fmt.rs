#[cfg(feature = "alloc")]
pub(crate) type OwnedDFA = DFA<alloc::vec::Vec<u32>>;
#[cfg(feature = "dfa-build")]
use core::cmp;
use core::{fmt, iter, mem::size_of, slice};
#[cfg(feature = "dfa-build")]
use alloc::{
    collections::{BTreeMap, BTreeSet},
    vec, vec::Vec,
};
#[cfg(feature = "dfa-build")]
use crate::{
    dfa::{accel::Accel, determinize, minimize::Minimizer, remapper::Remapper, sparse},
    nfa::thompson, util::{look::LookMatcher, search::MatchKind},
};
use crate::{
    dfa::{
        accel::Accels, automaton::{fmt_state_indicator, Automaton, StartError},
        special::Special, start::StartKind, DEAD,
    },
    util::{
        alphabet::{self, ByteClasses, ByteSet},
        int::{Pointer, Usize},
        prefilter::Prefilter, primitives::{PatternID, StateID},
        search::Anchored, start::{self, Start, StartByteMap},
        wire::{self, DeserializeError, Endian, SerializeError},
    },
};
const LABEL: &str = "rust-regex-automata-dfa-dense";
const VERSION: u32 = 2;
#[cfg(feature = "dfa-build")]
#[derive(Clone, Debug)]
pub struct BuildError {
    kind: BuildErrorKind,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Start {
    /// This occurs when the starting position is not any of the ones below.
    NonWordByte = 0,
    /// This occurs when the byte immediately preceding the start of the search
    /// is an ASCII word byte.
    WordByte = 1,
    /// This occurs when the starting position of the search corresponds to the
    /// beginning of the haystack.
    Text = 2,
    /// This occurs when the byte immediately preceding the start of the search
    /// is a line terminator. Specifically, `\n`.
    LineLF = 3,
    /// This occurs when the byte immediately preceding the start of the search
    /// is a line terminator. Specifically, `\r`.
    LineCR = 4,
    /// This occurs when a custom line terminator has been set via a
    /// `LookMatcher`, and when that line terminator is neither a `\r` or a
    /// `\n`.
    ///
    /// If the custom line terminator is a word byte, then this start
    /// configuration is still selected. DFAs that implement word boundary
    /// assertions will likely need to check whether the custom line terminator
    /// is a word byte, in which case, it should behave as if the byte
    /// satisfies `\b` in addition to multi-line anchors.
    CustomLineTerminator = 5,
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
#[derive(Clone, Debug)]
enum BuildErrorKind {
    NFA(nfa::thompson::BuildError),
    InsufficientCacheCapacity { minimum: usize, given: usize },
    InsufficientStateIDCapacity { err: LazyStateIDError },
    Unsupported(&'static str),
}
#[cfg(feature = "dfa-build")]
impl core::fmt::Display for BuildError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.kind() {
            BuildErrorKind::NFA(_) => write!(f, "error building NFA"),
            BuildErrorKind::Unsupported(ref msg) => {
                write!(f, "unsupported regex feature for DFAs: {}", msg)
            }
            BuildErrorKind::TooManyStates => {
                write!(f, "number of DFA states exceeds limit of {}", StateID::LIMIT,)
            }
            BuildErrorKind::TooManyStartStates => {
                let stride = Start::len();
                let max = usize::try_from(core::isize::MAX).unwrap();
                let limit = (max - stride) / stride;
                write!(
                    f,
                    "compiling DFA with start states exceeds pattern \
                     pattern limit of {}",
                    limit,
                )
            }
            BuildErrorKind::TooManyMatchPatternIDs => {
                write!(
                    f,
                    "compiling DFA with total patterns in all match states \
                 exceeds limit of {}",
                    PatternID::LIMIT,
                )
            }
            BuildErrorKind::DFAExceededSizeLimit { limit } => {
                write!(
                    f, "DFA exceeded size limit of {:?} during determinization", limit,
                )
            }
            BuildErrorKind::DeterminizeExceededSizeLimit { limit } => {
                write!(f, "determinization exceeded size limit of {:?}", limit)
            }
        }
    }
}
impl Start {
    pub(crate) fn from_usize(n: usize) -> Option<Start> {}
    pub(crate) fn len() -> usize {
        6
    }
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn as_u8(&self) -> u8 {}
    #[cfg_attr(feature = "perf-inline", inline(always))]
    pub(crate) fn as_usize(&self) -> usize {}
}
#[cfg(feature = "dfa-build")]
impl BuildError {
    fn kind(&self) -> &BuildErrorKind {
        &self.kind
    }
    pub(crate) fn nfa(err: thompson::BuildError) -> BuildError {}
    pub(crate) fn unsupported_dfa_word_boundary_unicode() -> BuildError {}
    pub(crate) fn too_many_states() -> BuildError {}
    pub(crate) fn too_many_start_states() -> BuildError {}
    pub(crate) fn too_many_match_pattern_ids() -> BuildError {}
    pub(crate) fn dfa_exceeded_size_limit(limit: usize) -> BuildError {}
    pub(crate) fn determinize_exceeded_size_limit(limit: usize) -> BuildError {}
}
