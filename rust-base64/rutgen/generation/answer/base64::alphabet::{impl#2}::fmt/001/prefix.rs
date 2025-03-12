// Answer 0

#[test]
fn test_fmt_reserved_byte_eq() {
    let error = ParseAlphabetError::ReservedByte(0x3D);
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

#[test]
fn test_fmt_reserved_byte_null() {
    let error = ParseAlphabetError::ReservedByte(0x00);
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

#[test]
fn test_fmt_reserved_byte_control_char() {
    let error = ParseAlphabetError::ReservedByte(0x1F);
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

#[test]
fn test_fmt_reserved_byte_delete() {
    let error = ParseAlphabetError::ReservedByte(0x7F);
    let mut buf = String::new();
    let _ = error.fmt(&mut buf);
}

