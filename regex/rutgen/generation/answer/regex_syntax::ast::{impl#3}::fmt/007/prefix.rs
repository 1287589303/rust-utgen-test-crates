// Answer 0

#[test]
fn test_repetition_missing() {
    let error_kind = ErrorKind::RepetitionMissing;
    let mut output = core::fmt::Formatter::new();
    let _ = error_kind.fmt(&mut output);
}

#[test]
fn test_repetition_count_invalid() {
    let error_kind = ErrorKind::RepetitionCountInvalid;
    let mut output = core::fmt::Formatter::new();
    let _ = error_kind.fmt(&mut output);
}

