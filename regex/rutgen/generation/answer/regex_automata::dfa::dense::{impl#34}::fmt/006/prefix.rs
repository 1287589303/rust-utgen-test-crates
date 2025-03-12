// Answer 0

#[test]
fn test_fmt_unsupported_regex_feature() {
    struct TestBuildError {
        kind: BuildErrorKind,
    }

    impl BuildError {
        fn new(kind: BuildErrorKind) -> Self {
            BuildError { kind }
        }

        fn kind(&self) -> &BuildErrorKind {
            &self.kind
        }
    }

    let error_message = "Unsupported regex feature";
    let build_error = TestBuildError::new(BuildErrorKind::Unsupported(error_message));

    let mut formatter = core::fmt::Formatter::default();
    let _result = build_error.fmt(&mut formatter);
}

#[test]
fn test_fmt_unsupported_regex_feature_empty_msg() {
    struct TestBuildError {
        kind: BuildErrorKind,
    }

    impl BuildError {
        fn new(kind: BuildErrorKind) -> Self {
            BuildError { kind }
        }

        fn kind(&self) -> &BuildErrorKind {
            &self.kind
        }
    }

    let error_message = "";
    let build_error = TestBuildError::new(BuildErrorKind::Unsupported(error_message));

    let mut formatter = core::fmt::Formatter::default();
    let _result = build_error.fmt(&mut formatter);
}

