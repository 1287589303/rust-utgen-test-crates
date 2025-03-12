// Answer 0

#[test]
fn test_put_slice_empty_src() {
    let mut bytes = BytesMut::with_capacity(10);
    let src: &[u8] = &[];
    bytes.put_slice(src);
}

#[test]
fn test_put_slice_exact_capacity_src() {
    let mut bytes = BytesMut::with_capacity(10);
    let src: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; // length equals capacity
    bytes.put_slice(src);
}

#[test]
fn test_put_slice_over_capacity_src() {
    let mut bytes = BytesMut::with_capacity(10);
    let src: &[u8] = &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]; // length exceeds capacity
    bytes.put_slice(src);
}

#[test]
fn test_put_slice_partial_src() {
    let mut bytes = BytesMut::with_capacity(10);
    let src: &[u8] = &[1, 2, 3, 4, 5]; // length is less than capacity
    bytes.put_slice(src);
}

