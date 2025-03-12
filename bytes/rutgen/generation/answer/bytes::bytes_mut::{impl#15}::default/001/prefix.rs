// Answer 0

#[test]
fn test_bytesmut_default() {
    let bytes_mut = BytesMut::default();
    let len = bytes_mut.len();
    let capacity = bytes_mut.capacity();
}

#[test]
fn test_bytesmut_new() {
    let bytes_mut = BytesMut::new();
    let len = bytes_mut.len();
    let capacity = bytes_mut.capacity();
}

#[test]
fn test_bytesmut_with_capacity_zero() {
    let capacity: usize = 0;
    let bytes_mut = BytesMut::with_capacity(capacity);
    let len = bytes_mut.len();
    let cap = bytes_mut.capacity();
}

#[test]
fn test_bytesmut_with_capacity_min() {
    let capacity: usize = 10;
    let bytes_mut = BytesMut::with_capacity(capacity);
    let len = bytes_mut.len();
    let cap = bytes_mut.capacity();
}

#[test]
fn test_bytesmut_with_capacity_max() {
    let capacity: usize = (1 << 17) - 1;
    let bytes_mut = BytesMut::with_capacity(capacity);
    let len = bytes_mut.len();
    let cap = bytes_mut.capacity();
}

#[test]
fn test_bytesmut_resize_zero() {
    let capacity: usize = 100;
    let mut bytes_mut = BytesMut::with_capacity(capacity);
    bytes_mut.resize(0, 0);
    let len = bytes_mut.len();
    let cap = bytes_mut.capacity();
}

#[test]
fn test_bytesmut_resize_to_capacity() {
    let capacity: usize = 100;
    let mut bytes_mut = BytesMut::with_capacity(capacity);
    bytes_mut.resize(capacity, 0);
    let len = bytes_mut.len();
    let cap = bytes_mut.capacity();
}

#[test]
fn test_bytesmut_resize_exceed_capacity() {
    let capacity: usize = 100;
    let mut bytes_mut = BytesMut::with_capacity(capacity);
    bytes_mut.resize(capacity + 1, 0);
    let len = bytes_mut.len();
    let cap = bytes_mut.capacity();
}

