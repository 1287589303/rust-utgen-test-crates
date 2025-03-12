// Answer 0

#[test]
fn test_truncate_to_zero() {
    let mut buf = BytesMut::with_capacity(10);
    buf.resize(10, 0);
    buf.truncate(0);
}

#[test]
fn test_truncate_to_length() {
    let mut buf = BytesMut::with_capacity(10);
    buf.resize(10, 0);
    buf.truncate(10);
}

#[test]
fn test_truncate_to_half_length() {
    let mut buf = BytesMut::with_capacity(10);
    buf.resize(10, 0);
    buf.truncate(5);
}

