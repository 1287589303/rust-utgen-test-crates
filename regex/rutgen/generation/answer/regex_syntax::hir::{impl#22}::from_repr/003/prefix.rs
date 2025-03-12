// Answer 0

#[test]
fn test_from_repr_valid_word_start_half_unicode() {
    let repr = 0b01_0000_0000_0000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_invalid_case_low() {
    let repr = 0b00_0000_0000_0000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_invalid_case_high() {
    let repr = 0b10_0000_0000_0000_0000;
    let result = Look::from_repr(repr);
}

