fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.kind {
            BuildErrorKind::NFA(_) => write!(f, "error building NFA"),
            BuildErrorKind::InsufficientCacheCapacity { minimum, given } => {
                write!(
                    f,
                    "given cache capacity ({}) is smaller than \
                     minimum required ({})",
                    given, minimum,
                )
            }
            BuildErrorKind::InsufficientStateIDCapacity { ref err } => {
                err.fmt(f)
            }
            BuildErrorKind::Unsupported(ref msg) => {
                write!(f, "unsupported regex feature for DFAs: {}", msg)
            }
        }
    }