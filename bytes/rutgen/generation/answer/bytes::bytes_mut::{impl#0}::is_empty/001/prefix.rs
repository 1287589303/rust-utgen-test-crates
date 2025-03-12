// Answer 0

#[test]
fn test_is_empty_zero_length() {
    let mut bytes_mut = BytesMut::new();
    assert!(bytes_mut.is_empty());
}

#[test]
fn test_is_empty_non_zero_length() {
    let mut bytes_mut = BytesMut::with_capacity(64);
    bytes_mut.resize(1, 0);
    assert!(!bytes_mut.is_empty());
}

#[test]
fn test_is_empty_max_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(usize::MAX);
    unsafe { bytes_mut.set_len(usize::MAX) }; // Directly setting length to max for testing
    assert!(!bytes_mut.is_empty());
}

