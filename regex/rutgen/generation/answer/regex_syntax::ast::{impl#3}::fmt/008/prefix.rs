// Answer 0

#[test]
fn test_repetition_count_unclosed() {
    let error_kind = super::ErrorKind::RepetitionCountUnclosed;
    let mut formatter = core::fmt::Formatter::new();
    let _ = error_kind.fmt(&mut formatter);
}

#[test]
fn test_repetition_count_unclosed_with_different_formatter() {
    let error_kind = super::ErrorKind::RepetitionCountUnclosed;
    let mut formatter = core::fmt::Formatter::new();
    let _ = error_kind.fmt(&mut formatter);
}

