// Answer 0

#[test]
fn test_from_repr_word_start_unicode() {
    let repr: u32 = 0b00_0001_0000_0000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_invalid() {
    let repr: u32 = 0b11_1111_1111_1111_1111;
    let result = Look::from_repr(repr);
}

