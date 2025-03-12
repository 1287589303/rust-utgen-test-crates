// Answer 0

#[test]
fn test_resize_increasing_length_zero_additional() {
    let mut buf = BytesMut::with_capacity(5);
    buf.resize(5, 0x1);
}

#[test]
fn test_resize_increasing_length_non_zero_additional() {
    let mut buf = BytesMut::with_capacity(5);
    buf.resize(3, 0x1);
}

#[test]
fn test_resize_equal_length() {
    let mut buf = BytesMut::with_capacity(5);
    buf.resize(3, 0x1);
    buf.resize(3, 0x2);
}

#[test]
fn test_resize_reducing_length() {
    let mut buf = BytesMut::with_capacity(5);
    buf.resize(5, 0x1);
    buf.resize(3, 0x2);
}

#[test]
fn test_resize_capacity() {
    let mut buf = BytesMut::with_capacity(5);
    buf.resize(5, 0x1);
    buf.resize(4, 0x2);
    buf.resize(5, 0x3);
}

