// Answer 0

#[test]
fn test_from_bytes_mut_empty() {
    let src = BytesMut::new();
    let _result: Bytes = Bytes::from(src);
}

#[test]
fn test_from_bytes_mut_zeroed_capacity() {
    let src = BytesMut::zeroed(10);
    let _result: Bytes = Bytes::from(src);
}

#[test]
fn test_from_bytes_mut_with_capacity() {
    let capacity = 1024; // an arbitrary non-zero capacity
    let src = BytesMut::with_capacity(capacity);
    let _result: Bytes = Bytes::from(src);
}

#[test]
fn test_from_bytes_mut_max_capacity() {
    let src = BytesMut::with_capacity(usize::MAX);
    let _result: Bytes = Bytes::from(src);
}

