#[cfg(feature = "std")]
type CaptureNameMap = std::collections::HashMap<Arc<str>, SmallIndex>;
#[cfg(not(feature = "std"))]
type CaptureNameMap = alloc::collections::BTreeMap<Arc<str>, SmallIndex>;
use alloc::{string::String, sync::Arc, vec, vec::Vec};
use crate::util::{
    interpolate,
    primitives::{NonMaxUsize, PatternID, PatternIDError, PatternIDIter, SmallIndex},
    search::{Match, Span},
};
#[derive(Clone, Debug)]
pub struct GroupInfoError {
    kind: GroupInfoErrorKind,
}
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
#[repr(transparent)]
pub struct PatternID(SmallIndex);
#[derive(Clone, Debug)]
enum GroupInfoErrorKind {
    /// This occurs when too many patterns have been added. i.e., It would
    /// otherwise overflow a `PatternID`.
    TooManyPatterns { err: PatternIDError },
    /// This occurs when too many capturing groups have been added for a
    /// particular pattern.
    TooManyGroups {
        /// The ID of the pattern that had too many groups.
        pattern: PatternID,
        /// The minimum number of groups that the caller has tried to add for
        /// a pattern.
        minimum: usize,
    },
    /// An error that occurs when a pattern has no capture groups. Either the
    /// group info must be empty, or all patterns must have at least one group
    /// (corresponding to the unnamed group for the entire pattern).
    MissingGroups {
        /// The ID of the pattern that had no capturing groups.
        pattern: PatternID,
    },
    /// An error that occurs when one tries to provide a name for the capture
    /// group at index 0. This capturing group must currently always be
    /// unnamed.
    FirstMustBeUnnamed {
        /// The ID of the pattern that was found to have a named first
        /// capturing group.
        pattern: PatternID,
    },
    /// An error that occurs when duplicate capture group names for the same
    /// pattern are added.
    ///
    /// NOTE: At time of writing, this error can never occur if you're using
    /// regex-syntax, since the parser itself will reject patterns with
    /// duplicate capture group names. This error can only occur when the
    /// builder is used to hand construct NFAs.
    Duplicate {
        /// The pattern in which the duplicate capture group name was found.
        pattern: PatternID,
        /// The duplicate name.
        name: String,
    },
}
impl core::fmt::Display for GroupInfoError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use self::GroupInfoErrorKind::*;
        match self.kind {
            TooManyPatterns { ref err } => {
                write!(f, "too many patterns to build capture info: {}", err)
            }
            TooManyGroups { pattern, minimum } => {
                write!(
                    f,
                    "too many capture groups (at least {}) were \
                     found for pattern {}",
                    minimum, pattern.as_usize()
                )
            }
            MissingGroups { pattern } => {
                write!(
                    f,
                    "no capturing groups found for pattern {} \
                 (either all patterns have zero groups or all patterns have \
                  at least one group)",
                    pattern.as_usize(),
                )
            }
            FirstMustBeUnnamed { pattern } => {
                write!(
                    f,
                    "first capture group (at index 0) for pattern {} has a name \
                 (it must be unnamed)",
                    pattern.as_usize(),
                )
            }
            Duplicate { pattern, ref name } => {
                write!(
                    f, "duplicate capture group name '{}' found for pattern {}", name,
                    pattern.as_usize(),
                )
            }
        }
    }
}
