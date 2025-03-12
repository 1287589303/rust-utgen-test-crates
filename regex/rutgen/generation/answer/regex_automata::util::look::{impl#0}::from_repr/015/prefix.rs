// Answer 0

#[test]
fn test_from_repr_start() {
    let repr = 0b00_0000_0000_0000_0001;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_start_crlf() {
    let repr = 0b00_0000_0000_0001_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_end() {
    let repr = 0b00_0000_0000_0000_0010;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_start_lf() {
    let repr = 0b00_0000_0000_0000_0100;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_end_lf() {
    let repr = 0b00_0000_0000_0000_1000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_end_crlf() {
    let repr = 0b00_0000_0000_0010_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_word_ascii() {
    let repr = 0b00_0000_0000_0100_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_word_ascii_negate() {
    let repr = 0b00_0000_0000_1000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_word_unicode() {
    let repr = 0b00_0000_0001_0000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_word_unicode_negate() {
    let repr = 0b00_0000_0010_0000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_word_start_ascii() {
    let repr = 0b00_0000_0100_0000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_word_end_ascii() {
    let repr = 0b00_0000_1000_0000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_word_start_unicode() {
    let repr = 0b00_0001_0000_0000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_word_end_unicode() {
    let repr = 0b00_0010_0000_0000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_word_start_half_ascii() {
    let repr = 0b00_0100_0000_0000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_word_end_half_ascii() {
    let repr = 0b00_1000_0000_0000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_word_start_half_unicode() {
    let repr = 0b01_0000_0000_0000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_word_end_half_unicode() {
    let repr = 0b10_0000_0000_0000_0000;
    let result = Look::from_repr(repr);
}

