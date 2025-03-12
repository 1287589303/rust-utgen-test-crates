// Answer 0

#[test]
fn test_from_repr_word_start_half_ascii() {
    let repr: u32 = 0b00_0100_0000_0000_0000; // Matches WordStartHalfAscii
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_start() {
    let repr: u32 = 0b00_0000_0000_0000_0001; // Matches Start
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_end() {
    let repr: u32 = 0b00_0000_0000_0000_0010; // Matches End
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_start_lf() {
    let repr: u32 = 0b00_0000_0000_0000_0100; // Matches StartLF
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_end_lf() {
    let repr: u32 = 0b00_0000_0000_0000_1000; // Matches EndLF
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_invalid_zero() {
    let repr: u32 = 0; // Invalid representation
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_invalid_greater_than_max() {
    let repr: u32 = 0b11; // Invalid representation, greater than defined max
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_invalid_single_bit() {
    let repr: u32 = 0b01_0000_0000_0000_0000; // Invalid representation
    let result = Look::from_repr(repr);
}

