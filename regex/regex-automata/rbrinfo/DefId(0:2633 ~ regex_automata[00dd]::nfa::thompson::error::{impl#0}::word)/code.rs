pub(crate) fn word(err: look::UnicodeWordBoundaryError) -> BuildError {
        BuildError { kind: BuildErrorKind::Word(err) }
    }