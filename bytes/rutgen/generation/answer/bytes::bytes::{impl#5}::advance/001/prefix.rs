// Answer 0

#[test]
fn test_advance_exact_length() {
    let mut bytes = Bytes::from_static(b"test");
    let length = bytes.len();
    bytes.advance(length);
}

#[test]
fn test_advance_zero() {
    let mut bytes = Bytes::from_static(b"test");
    bytes.advance(0);
}

