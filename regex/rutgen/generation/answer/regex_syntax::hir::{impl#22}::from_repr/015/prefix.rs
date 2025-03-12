// Answer 0

#[test]
fn test_from_repr_valid_start_crlf() {
    let repr: u32 = 0b00_0000_0000_0001_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_start() {
    let repr: u32 = 0b00_0000_0000_0000_0001;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_invalid() {
    let repr: u32 = 0b00_0000_0000_0000_0000; 
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_out_of_range() {
    let repr: u32 = 0b10_0000_0000_0000_0001; 
    let result = Look::from_repr(repr);
}

