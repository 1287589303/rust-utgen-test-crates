// Answer 0

#[test]
fn test_from_repr_word_unicode() {
    let repr = 0b00_0000_0001_0000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_start() {
    let repr = 0b00_0000_0000_0000_0001;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_end() {
    let repr = 0b00_0000_0000_0000_0010;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_endlf() {
    let repr = 0b00_0000_0000_0000_1000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_invalid() {
    let repr = 0b11_1111_1111_1111_1111;
    let result = Look::from_repr(repr);
}

