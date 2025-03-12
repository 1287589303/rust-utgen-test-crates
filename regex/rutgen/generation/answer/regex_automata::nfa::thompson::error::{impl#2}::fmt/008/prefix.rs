// Answer 0

#[test]
fn test_fmt_syntax_error() {
    let error = BuildError {
        kind: BuildErrorKind::Syntax(regex_syntax::Error::new("error".to_string())),
    };
    let _ = core::fmt::format(format_args!("{}", error));
}

#[test]
fn test_fmt_captures_error() {
    let error = BuildError {
        kind: BuildErrorKind::Captures(captures::GroupInfoError::new("error".to_string())),
    };
    let _ = core::fmt::format(format_args!("{}", error));
}

#[test]
fn test_fmt_word_boundary_error() {
    let error = BuildError {
        kind: BuildErrorKind::Word(look::UnicodeWordBoundaryError::new("error".to_string())),
    };
    let _ = core::fmt::format(format_args!("{}", error));
}

#[test]
fn test_fmt_too_many_patterns() {
    let error = BuildError {
        kind: BuildErrorKind::TooManyPatterns { given: 10, limit: 5 },
    };
    let _ = core::fmt::format(format_args!("{}", error));
}

#[test]
fn test_fmt_too_many_states() {
    let error = BuildError {
        kind: BuildErrorKind::TooManyStates { given: 10, limit: 5 },
    };
    let _ = core::fmt::format(format_args!("{}", error));
}

#[test]
fn test_fmt_exceeded_size_limit() {
    let error = BuildError {
        kind: BuildErrorKind::ExceededSizeLimit { limit: 1024 },
    };
    let _ = core::fmt::format(format_args!("{}", error));
}

#[test]
fn test_fmt_invalid_capture_index() {
    let error = BuildError {
        kind: BuildErrorKind::InvalidCaptureIndex { index: u32::MAX },
    };
    let _ = core::fmt::format(format_args!("{}", error));
}

#[test]
fn test_fmt_unsupported_captures() {
    let error = BuildError {
        kind: BuildErrorKind::UnsupportedCaptures,
    };
    let _ = core::fmt::format(format_args!("{}", error));
}

