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
                    minimum,
                    pattern.as_usize()
                )
            }
            MissingGroups { pattern } => write!(
                f,
                "no capturing groups found for pattern {} \
                 (either all patterns have zero groups or all patterns have \
                  at least one group)",
                pattern.as_usize(),
            ),
            FirstMustBeUnnamed { pattern } => write!(
                f,
                "first capture group (at index 0) for pattern {} has a name \
                 (it must be unnamed)",
                pattern.as_usize(),
            ),
            Duplicate { pattern, ref name } => write!(
                f,
                "duplicate capture group name '{}' found for pattern {}",
                name,
                pattern.as_usize(),
            ),
        }
    }