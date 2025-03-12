// Answer 0

#[test]
fn test_fmt_nfa() {
    let error_instance = BuildError {
        kind: BuildErrorKind::NFA(nfa::thompson::BuildError::from("valid input")),
    };
    let mut formatter = core::fmt::Formatter::new();
    let _ = error_instance.fmt(&mut formatter);
}

#[test]
fn test_fmt_nfa_empty_pattern() {
    let error_instance = BuildError {
        kind: BuildErrorKind::NFA(nfa::thompson::BuildError::from("")),
    };
    let mut formatter = core::fmt::Formatter::new();
    let _ = error_instance.fmt(&mut formatter);
}

#[test]
fn test_fmt_nfa_invalid_syntax() {
    let invalid_syntax_pattern = "([a-z"; // incomplete regex
    let error_instance = BuildError {
        kind: BuildErrorKind::NFA(nfa::thompson::BuildError::from(invalid_syntax_pattern)),
    };
    let mut formatter = core::fmt::Formatter::new();
    let _ = error_instance.fmt(&mut formatter);
}

