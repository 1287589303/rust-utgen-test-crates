// Answer 0

#[test]
fn test_bytesmut_new_empty() {
    let bytes = BytesMut::new();
}

#[test]
fn test_bytesmut_new_length_zero() {
    let bytes = BytesMut::new();
    let len = bytes.len();
}

#[test]
fn test_bytesmut_new_capacity_zero() {
    let bytes = BytesMut::new();
    let cap = bytes.capacity();
}

#[test]
fn test_bytesmut_new_no_allocation() {
    let bytes = BytesMut::new();
    let initial_ptr = bytes.ptr.as_ptr();
    let new_bytes = BytesMut::new();
    let new_ptr = new_bytes.ptr.as_ptr();
    assert_eq!(initial_ptr, new_ptr);
}

