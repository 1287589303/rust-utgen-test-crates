// Answer 0

#[test]
fn test_from_repr_start() {
    let result = Look::from_repr(0b00_0000_0000_0000_0001);
}

#[test]
fn test_from_repr_end() {
    let result = Look::from_repr(0b00_0000_0000_0000_0010);
}

#[test]
fn test_from_repr_start_lf() {
    let result = Look::from_repr(0b00_0000_0000_0000_0100);
}

#[test]
fn test_from_repr_end_lf() {
    let result = Look::from_repr(0b00_0000_0000_0000_1000);
}

#[test]
fn test_from_repr_start_crlf() {
    let result = Look::from_repr(0b00_0000_0000_0001_0000);
}

#[test]
fn test_from_repr_end_crlf() {
    let result = Look::from_repr(0b00_0000_0000_0010_0000);
}

#[test]
fn test_from_repr_word_unicode() {
    let result = Look::from_repr(0b00_0000_0001_0000_0000);
}

#[test]
fn test_from_repr_word_ascii() {
    let result = Look::from_repr(0b00_0000_0100_0000_0000);
}

#[test]
fn test_from_repr_word_ascii_negate() {
    let result = Look::from_repr(0b00_0000_1000_0000_0000);
}

#[test]
fn test_from_repr_word_unicode_negate() {
    let result = Look::from_repr(0b00_0000_0010_0000_0000);
}

#[test]
fn test_from_repr_word_start_ascii() {
    let result = Look::from_repr(0b00_0000_0100_0000_0000);
}

#[test]
fn test_from_repr_word_end_ascii() {
    let result = Look::from_repr(0b00_0000_1000_0000_0000);
}

#[test]
fn test_from_repr_word_start_half_unicode() {
    let result = Look::from_repr(0b01_0000_0000_0000_0000);
}

#[test]
fn test_from_repr_word_end_half_unicode() {
    let result = Look::from_repr(0b10_0000_0000_0000_0000);
}

#[test]
fn test_from_repr_other() {
    let result = Look::from_repr(0b11_1111_1111_1111_1111);
}

