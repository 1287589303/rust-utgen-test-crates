fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.kind() {
            BuildErrorKind::NFA(_) => write!(f, "error building NFA"),
            BuildErrorKind::Unsupported(ref msg) => {
                write!(f, "unsupported regex feature for DFAs: {}", msg)
            }
            BuildErrorKind::TooManyStates => write!(
                f,
                "number of DFA states exceeds limit of {}",
                StateID::LIMIT,
            ),
            BuildErrorKind::TooManyStartStates => {
                let stride = Start::len();
                // The start table has `stride` entries for starting states for
                // the entire DFA, and then `stride` entries for each pattern
                // if start states for each pattern are enabled (which is the
                // only way this error can occur). Thus, the total number of
                // patterns that can fit in the table is `stride` less than
                // what we can allocate.
                let max = usize::try_from(core::isize::MAX).unwrap();
                let limit = (max - stride) / stride;
                write!(
                    f,
                    "compiling DFA with start states exceeds pattern \
                     pattern limit of {}",
                    limit,
                )
            }
            BuildErrorKind::TooManyMatchPatternIDs => write!(
                f,
                "compiling DFA with total patterns in all match states \
                 exceeds limit of {}",
                PatternID::LIMIT,
            ),
            BuildErrorKind::DFAExceededSizeLimit { limit } => write!(
                f,
                "DFA exceeded size limit of {:?} during determinization",
                limit,
            ),
            BuildErrorKind::DeterminizeExceededSizeLimit { limit } => {
                write!(f, "determinization exceeded size limit of {:?}", limit)
            }
        }
    }