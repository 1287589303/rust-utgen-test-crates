// Answer 0

#[test]
fn test_from_repr_valid_end_crlf() {
    let repr: u32 = 0b00_0000_0000_0010_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_start() {
    let repr: u32 = 0b00_0000_0000_0000_0001;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_end() {
    let repr: u32 = 0b00_0000_0000_0000_0010;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_startlf() {
    let repr: u32 = 0b00_0000_0000_0000_0100;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_endlf() {
    let repr: u32 = 0b00_0000_0000_0000_1000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_startcrlf() {
    let repr: u32 = 0b00_0000_0000_0001_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_wordascii() {
    let repr: u32 = 0b00_0000_0000_0100_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_wordascii_negate() {
    let repr: u32 = 0b00_0000_0000_1000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_wordunicode() {
    let repr: u32 = 0b00_0000_0001_0000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_wordunicode_negate() {
    let repr: u32 = 0b00_0000_0010_0000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_wordstartascii() {
    let repr: u32 = 0b00_0000_0100_0000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_wordendascii() {
    let repr: u32 = 0b00_0000_1000_0000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_wordstartunicode() {
    let repr: u32 = 0b00_0001_0000_0000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_wordendunicode() {
    let repr: u32 = 0b00_0010_0000_0000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_wordstarthalfascii() {
    let repr: u32 = 0b00_0100_0000_0000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_wordendhalfascii() {
    let repr: u32 = 0b00_1000_0000_0000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_wordstarthalfunicode() {
    let repr: u32 = 0b01_0000_0000_0000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_valid_wordendhalfunicode() {
    let repr: u32 = 0b10_0000_0000_0000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_invalid() {
    let repr: u32 = 0b1111_1111_1111_1111_1111;
    let result = Look::from_repr(repr);
}

