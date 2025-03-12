// Answer 0

#[test]
fn test_fmt_too_many_patterns_zero() {
    let error = BuildError {
        kind: BuildErrorKind::TooManyPatterns { limit: 0 },
    };
    let mut formatter = core::fmt::Formatter::new();
    let _ = error.fmt(&mut formatter);
}

#[test]
fn test_fmt_too_many_patterns_mid() {
    let error = BuildError {
        kind: BuildErrorKind::TooManyPatterns { limit: 500 },
    };
    let mut formatter = core::fmt::Formatter::new();
    let _ = error.fmt(&mut formatter);
}

#[test]
fn test_fmt_too_many_patterns_high() {
    let error = BuildError {
        kind: BuildErrorKind::TooManyPatterns { limit: 1000 },
    };
    let mut formatter = core::fmt::Formatter::new();
    let _ = error.fmt(&mut formatter);
}

