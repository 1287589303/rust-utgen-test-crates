// Answer 0

#[test]
fn test_fmt_nfa_build_error() {
    #[derive(Clone, Debug)]
    struct DummyThompsonError;

    let build_error = BuildError {
        kind: BuildErrorKind::NFA(DummyThompsonError),
    };

    let mut formatter = core::fmt::Formatter::new();
    let _ = build_error.fmt(&mut formatter);
}

#[test]
fn test_fmt_nfa_with_specific_values() {
    #[derive(Clone, Debug)]
    struct SpecificThompsonError;

    let build_error = BuildError {
        kind: BuildErrorKind::NFA(SpecificThompsonError),
    };

    let mut formatter = core::fmt::Formatter::new();
    let _ = build_error.fmt(&mut formatter);
}

