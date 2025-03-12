// Answer 0

#[test]
fn test_special_word_or_repetition_unexpected_eof_unclosed_brace() {
    let error = ErrorKind::SpecialWordOrRepetitionUnexpectedEof;
    let mut formatter = core::fmt::Formatter::new();
    let _ = error.fmt(&mut formatter);
}

#[test]
fn test_special_word_or_repetition_unexpected_eof_end_of_pattern() {
    let error = ErrorKind::SpecialWordOrRepetitionUnexpectedEof;
    let mut formatter = core::fmt::Formatter::new();
    let _ = error.fmt(&mut formatter);
}

