fn word(err: UnicodeWordBoundaryError) -> BuildError {
        BuildError { kind: BuildErrorKind::Word(err) }
    }