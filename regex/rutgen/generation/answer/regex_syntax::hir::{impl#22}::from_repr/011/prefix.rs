// Answer 0

#[test]
fn test_from_repr_word_unicode() {
    let repr: u32 = 0b00_0000_0001_0000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_invalid_none() {
    let repr: u32 = 0b11_1111_1111_1111_1111; // invalid representation
    let result = Look::from_repr(repr);
}

