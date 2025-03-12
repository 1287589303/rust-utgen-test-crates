pub(crate) fn unsupported_captures() -> BuildError {
        BuildError { kind: BuildErrorKind::UnsupportedCaptures }
    }