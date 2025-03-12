// Answer 0

#[test]
fn test_resize_increase_length_with_non_zero_value() {
    let mut buf = BytesMut::with_capacity(5);
    buf.resize(3, 0x1);
}

#[test]
fn test_resize_increase_length_with_value_zero() {
    let mut buf = BytesMut::with_capacity(5);
    buf.resize(4, 0x0);
}

#[test]
fn test_resize_increase_length_with_high_value() {
    let mut buf = BytesMut::with_capacity(5);
    buf.resize(6, 0xFF);
}

