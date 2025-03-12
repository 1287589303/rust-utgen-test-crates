// Answer 0

#[test]
fn test_fmt_word_boundary_error() {
    struct TestBuildError {
        kind: BuildErrorKind,
    }

    let error_kind = BuildErrorKind::Word(UnicodeWordBoundaryError {});
    let build_error = TestBuildError { kind: error_kind };

    let mut formatter = core::fmt::Formatter::default();
    let _ = build_error.fmt(&mut formatter);
}

#[test]
fn test_fmt_word_boundary_error_with_message() {
    struct TestBuildError {
        kind: BuildErrorKind,
    }

    let error_kind = BuildErrorKind::Word(UnicodeWordBoundaryError {
        msg: "Invalid boundary".to_string(),
    });
    let build_error = TestBuildError { kind: error_kind };

    let mut formatter = core::fmt::Formatter::default();
    let _ = build_error.fmt(&mut formatter);
}

