fn unsupported_look(look: Look) -> BuildError {
        BuildError { kind: BuildErrorKind::UnsupportedLook { look } }
    }