// Answer 0

#[test]
fn test_extend_from_slice_with_exact_capacity() {
    let mut buf = BytesMut::with_capacity(6);
    buf.resize(6, 0);
    buf.extend_from_slice(b"abc");
}

#[test]
fn test_extend_from_slice_with_greater_capacity() {
    let mut buf = BytesMut::with_capacity(10);
    buf.resize(4, 0);
    buf.extend_from_slice(b"abcd");
}

#[test]
fn test_extend_from_slice_with_zero_length() {
    let mut buf = BytesMut::with_capacity(5);
    buf.resize(2, 0);
    buf.extend_from_slice(b"");
}

#[test]
fn test_extend_from_slice_with_large_capacity() {
    let mut buf = BytesMut::with_capacity(50);
    buf.resize(20, 0);
    buf.extend_from_slice(b"123456");
}

