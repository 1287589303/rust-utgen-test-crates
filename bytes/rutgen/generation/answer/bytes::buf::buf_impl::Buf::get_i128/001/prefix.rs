// Answer 0

#[test]
fn test_get_i128_exact_length() {
    let mut buf = &b"\x01\x02\x03\x04\x05\x06\x07\x08\x09\x10\x11\x12\x13\x14\x15\x16"[..];
    let value = buf.get_i128();
}

#[test]
fn test_get_i128_more_than_minimum_length() {
    let mut buf = &b"\x01\x02\x03\x04\x05\x06\x07\x08\x09\x10\x11\x12\x13\x14\x15\x16 additional data"[..];
    let value = buf.get_i128();
}

#[should_panic]
#[test]
fn test_get_i128_insufficient_length() {
    let mut buf = &b"\x01\x02\x03\x04\x05\x06\x07\x08"[..];
    let value = buf.get_i128();
}

