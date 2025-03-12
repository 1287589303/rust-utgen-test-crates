// Answer 0

#[test]
fn test_fmt_too_many_states() {
    let limit = StateID::LIMIT;
    let error = BuildError {
        kind: BuildErrorKind::TooManyStates { limit },
    };
    
    let mut buffer = core::fmt::Formatter::new();
    let _ = error.fmt(&mut buffer);
}

#[test]
fn test_fmt_too_many_states_boundary() {
    let limit = StateID::LIMIT - 1;
    let error = BuildError {
        kind: BuildErrorKind::TooManyStates { limit },
    };
    
    let mut buffer = core::fmt::Formatter::new();
    let _ = error.fmt(&mut buffer);
}

#[test]
fn test_fmt_too_many_states_exceeds() {
    let limit = StateID::LIMIT + 1;
    // Note: this should panic or give an invalid case, so we use #[should_panic].
    let error = BuildError {
        kind: BuildErrorKind::TooManyStates { limit },
    };
    
    let mut buffer = core::fmt::Formatter::new();
    let _ = error.fmt(&mut buffer);
}

