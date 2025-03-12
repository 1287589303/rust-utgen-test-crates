// Answer 0

#[test]
fn test_from_repr_valid_word_start_half_ascii() {
    let repr: u32 = 0b00_0100_0000_0000_0000; // matches 0b00_0100_0000_0000_0000
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_invalid_high_value() {
    let repr: u32 = 0b11111111111111111111; // invalid representation
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_invalid_zero() {
    let repr: u32 = 0b00000000000000000000; // invalid representation
    let result = Look::from_repr(repr);
}

