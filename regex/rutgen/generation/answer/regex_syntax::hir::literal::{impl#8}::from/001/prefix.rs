// Answer 0

#[test]
fn test_from_byte_0() {
    let byte: u8 = 0;
    let _literal = Literal::from(byte);
}

#[test]
fn test_from_byte_1() {
    let byte: u8 = 1;
    let _literal = Literal::from(byte);
}

#[test]
fn test_from_byte_255() {
    let byte: u8 = 255;
    let _literal = Literal::from(byte);
}

#[test]
fn test_from_byte_mid_range() {
    let byte: u8 = 128;
    let _literal = Literal::from(byte);
}

