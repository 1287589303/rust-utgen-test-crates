// Answer 0

#[test]
fn test_from_repr_end_crlf() {
    let repr: u32 = 0b00_0000_0000_0010_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_invalid() {
    let repr: u32 = 0b00_0000_0000_0000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_invalid_high() {
    let repr: u32 = 0b10_0000_0000_0000_0000;
    let result = Look::from_repr(repr);
}

