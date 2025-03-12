fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use self::BuildErrorKind::*;

        match self.kind {
            NFA(_) => write!(f, "error building NFA"),
            Word(_) => write!(f, "NFA contains Unicode word boundary"),
            TooManyStates { limit } => write!(
                f,
                "one-pass DFA exceeded a limit of {:?} for number of states",
                limit,
            ),
            TooManyPatterns { limit } => write!(
                f,
                "one-pass DFA exceeded a limit of {:?} for number of patterns",
                limit,
            ),
            UnsupportedLook { look } => write!(
                f,
                "one-pass DFA does not support the {:?} assertion",
                look,
            ),
            ExceededSizeLimit { limit } => write!(
                f,
                "one-pass DFA exceeded size limit of {:?} during building",
                limit,
            ),
            NotOnePass { msg } => write!(
                f,
                "one-pass DFA could not be built because \
                 pattern is not one-pass: {}",
                msg,
            ),
        }
    }