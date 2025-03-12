// Answer 0

#[test]
fn test_from_repr_word_end_half_ascii() {
    let repr: u32 = 0b00_1000_0000_0000_0000; // 32768
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_invalid() {
    let repr: u32 = 0b00_0000_0000_0000_0001; // 1
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_another_invalid() {
    let repr: u32 = 0b11_1111_1111_1111_1111; // 65535
    let result = Look::from_repr(repr);
}

