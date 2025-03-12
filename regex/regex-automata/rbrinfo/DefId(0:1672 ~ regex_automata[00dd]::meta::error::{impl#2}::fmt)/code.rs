fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self.kind {
            BuildErrorKind::Syntax { pid, .. } => {
                write!(f, "error parsing pattern {}", pid.as_usize())
            }
            BuildErrorKind::NFA(_) => write!(f, "error building NFA"),
        }
    }