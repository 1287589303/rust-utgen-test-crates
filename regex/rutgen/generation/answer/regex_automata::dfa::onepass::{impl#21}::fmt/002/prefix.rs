// Answer 0

#[test]
fn test_fmt_exceeded_size_limit() {
    let build_error = BuildError {
        kind: BuildErrorKind::ExceededSizeLimit { limit: usize::MAX },
    };
    let mut buffer = Vec::new();
    let _ = build_error.fmt(&mut core::fmt::Formatter::new(&mut buffer));
}

#[test]
fn test_fmt_exceeded_size_limit_zero() {
    let build_error = BuildError {
        kind: BuildErrorKind::ExceededSizeLimit { limit: 0 },
    };
    let mut buffer = Vec::new();
    let _ = build_error.fmt(&mut core::fmt::Formatter::new(&mut buffer));
}

#[test]
fn test_fmt_exceeded_size_limit_large_value() {
    let build_error = BuildError {
        kind: BuildErrorKind::ExceededSizeLimit { limit: 1024 },
    };
    let mut buffer = Vec::new();
    let _ = build_error.fmt(&mut core::fmt::Formatter::new(&mut buffer));
}

