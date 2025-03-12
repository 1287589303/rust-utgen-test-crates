// Answer 0

#[test]
fn test_truncate_equal_length() {
    let mut buf = Bytes::copy_from_slice(&[1, 2, 3, 4, 5]);
    buf.truncate(buf.len());
}

#[test]
fn test_truncate_greater_length() {
    let mut buf = Bytes::copy_from_slice(&[1, 2, 3, 4, 5]);
    buf.truncate(10);
}

#[test]
fn test_truncate_empty() {
    let mut buf = Bytes::new();
    buf.truncate(0);
}

#[test]
#[should_panic]
fn test_truncate_invalid_length() {
    let mut buf = Bytes::copy_from_slice(&[1, 2, 3]);
    buf.truncate(usize::MAX);
}

