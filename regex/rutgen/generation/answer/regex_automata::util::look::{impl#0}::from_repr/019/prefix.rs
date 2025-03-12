// Answer 0

#[test]
fn test_from_repr_start() {
    let repr: u32 = 0b00_0000_0000_0000_0001;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_end() {
    let repr: u32 = 0b00_0000_0000_0000_0010;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_startlf() {
    let repr: u32 = 0b00_0000_0000_0000_0100;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_endlf() {
    let repr: u32 = 0b00_0000_0000_0000_1000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_startcrlf() {
    let repr: u32 = 0b00_0000_0000_0001_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_endcrlf() {
    let repr: u32 = 0b00_0000_0000_0010_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_wordascii() {
    let repr: u32 = 0b00_0000_0000_0100_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_wordasciinegate() {
    let repr: u32 = 0b00_0000_0000_1000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_wordunicode() {
    let repr: u32 = 0b00_0000_0001_0000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_wordunicodenegate() {
    let repr: u32 = 0b00_0000_0010_0000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_wordstartascii() {
    let repr: u32 = 0b00_0000_0100_0000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_wordendascii() {
    let repr: u32 = 0b00_0000_1000_0000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_wordstartunicode() {
    let repr: u32 = 0b00_0001_0000_0000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_wordendunicode() {
    let repr: u32 = 0b00_0010_0000_0000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_wordstarthalfascii() {
    let repr: u32 = 0b00_0100_0000_0000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_wordendhalfascii() {
    let repr: u32 = 0b00_1000_0000_0000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_wordstarthalfunicode() {
    let repr: u32 = 0b01_0000_0000_0000_0000;
    let result = Look::from_repr(repr);
}

#[test]
fn test_from_repr_wordendhalfunicode() {
    let repr: u32 = 0b10_0000_0000_0000_0000;
    let result = Look::from_repr(repr);
}

