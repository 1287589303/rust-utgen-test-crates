// Answer 0

#[test]
fn test_extend_from_slice_insufficient_capacity() {
    let mut buf = BytesMut::with_capacity(5); // initial capacity is less than the extend length
    buf.extend_from_slice(b"abcde"); // extend with length greater than capacity
}

#[test]
fn test_extend_from_slice_with_exact_capacity() {
    let mut buf = BytesMut::with_capacity(5); // initial capacity
    buf.extend_from_slice(b"abc"); // fill part of the capacity
    buf.extend_from_slice(b"de"); // now extending to hit the exact capacity
}

#[test]
fn test_extend_from_slice_large_extension() {
    let mut buf = BytesMut::new(); // Starts with 0 capacity
    buf.extend_from_slice(b"hello"); // Extend with a length greater than initial capacity
}

