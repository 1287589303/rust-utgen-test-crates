// Answer 0

#[test]
fn test_truncate_len_greater_than_current_length() {
    let mut buf = BytesMut::with_capacity(10);
    buf.resize(5, 0); // Current length = 5
    buf.truncate(6); // len = 6, which is greater than current length
}

#[test]
fn test_truncate_len_equal_to_current_length() {
    let mut buf = BytesMut::with_capacity(10);
    buf.resize(5, 0); // Current length = 5
    buf.truncate(5); // len = 5, equal to current length
}

#[test]
fn test_truncate_len_zero() {
    let mut buf = BytesMut::with_capacity(10);
    buf.resize(5, 0); // Current length = 5
    buf.truncate(0); // len = 0, which is less than current length
}

#[test]
fn test_truncate_len_one_greater_than_current_length() {
    let mut buf = BytesMut::with_capacity(10);
    buf.resize(5, 0); // Current length = 5
    buf.truncate(6); // len = 6, which is greater than current length
}

