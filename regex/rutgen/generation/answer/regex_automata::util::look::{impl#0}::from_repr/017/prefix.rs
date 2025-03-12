// Answer 0

#[test]
fn test_from_repr_start_lf() {
    let repr = 0b00_0000_0000_0000_0100;
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
fn test_from_repr_non_matching() {
    let repr = 0b11_1111_1111_1111_1111; // Invalid representation
    let result = Look::from_repr(repr);
}

