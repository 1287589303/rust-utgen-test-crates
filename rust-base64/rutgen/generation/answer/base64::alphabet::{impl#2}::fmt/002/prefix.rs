// Answer 0

#[test]
fn test_unprintable_byte_low_boundary() {
    let error = ParseAlphabetError::UnprintableByte(0);
    let mut buffer = core::fmt::Formatter::new();
    let _ = error.fmt(&mut buffer);
}

#[test]
fn test_unprintable_byte_high_boundary() {
    let error = ParseAlphabetError::UnprintableByte(255);
    let mut buffer = core::fmt::Formatter::new();
    let _ = error.fmt(&mut buffer);
}

#[test]
fn test_unprintable_byte_middle_range() {
    let error = ParseAlphabetError::UnprintableByte(31);
    let mut buffer = core::fmt::Formatter::new();
    let _ = error.fmt(&mut buffer);
}

#[test]
fn test_unprintable_byte_another_high_range() {
    let error = ParseAlphabetError::UnprintableByte(127);
    let mut buffer = core::fmt::Formatter::new();
    let _ = error.fmt(&mut buffer);
}

