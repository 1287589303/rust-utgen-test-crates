// Answer 0

#[test]
fn test_from_repr_word_end_half_unicode() {
    let repr: u32 = 0b10_0000_0000_0000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_invalid() {
    let repr: u32 = 0b10_0000_0000_0000_0001;
    let result = Look::from_repr(repr);
} 

#[test]
fn test_from_repr_zero() {
    let repr: u32 = 0b00_0000_0000_0000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_high_value() {
    let repr: u32 = 0b11_1111_1111_1111_1111;
    let result = Look::from_repr(repr);
} 

