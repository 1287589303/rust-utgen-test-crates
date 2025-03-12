// Answer 0

#[test]
fn test_build_error_fmt_too_many_states_min_limit() {
    let error = BuildError {
        kind: BuildErrorKind::TooManyStates { limit: 1 },
    };
    let mut output = core::fmt::Formatter::new();
    let _ = error.fmt(&mut output);
}

#[test]
fn test_build_error_fmt_too_many_states_mid_limit() {
    let error = BuildError {
        kind: BuildErrorKind::TooManyStates { limit: 500 },
    };
    let mut output = core::fmt::Formatter::new();
    let _ = error.fmt(&mut output);
}

#[test]
fn test_build_error_fmt_too_many_states_max_limit() {
    let error = BuildError {
        kind: BuildErrorKind::TooManyStates { limit: u64::MAX as usize },
    };
    let mut output = core::fmt::Formatter::new();
    let _ = error.fmt(&mut output);
}

