// Answer 0

#[test]
fn test_from_repr_valid_word_start_ascii() {
    let repr: u32 = 0b00_0000_0100_0000_0000;
    let _result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_invalid_zero() {
    let repr: u32 = 0;
    let _result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_invalid_high_bit() {
    let repr: u32 = 0b01_0000_0000_0000_0000;
    let _result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_invalid_another_high_bit() {
    let repr: u32 = 0b10_0000_0000_0000_0000;
    let _result = Look::from_repr(repr);
}

