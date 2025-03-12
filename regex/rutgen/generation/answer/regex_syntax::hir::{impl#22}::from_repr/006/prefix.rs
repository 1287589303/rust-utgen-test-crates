// Answer 0

#[test]
fn test_from_repr_word_end_unicode() {
    let repr: u32 = 0b00_0010_0000_0000_0000; // equivalent to 512 in decimal
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_word_end_unicode_hex() {
    let repr: u32 = 0x200; // hexadecimal representation
    let result = Look::from_repr(repr);
}

