// Answer 0

#[test]
fn test_bytes_mut_reserve_no_allocation() {
    let mut buf = BytesMut::with_capacity(128);
    buf.resize(64, 0); // current length 64, capacity 128
    let rem = buf.capacity() - buf.len(); // rem = 64
    buf.reserve(rem); // additional = rem, should not allocate
}

#[test]
fn test_bytes_mut_reserve_exact_capacity() {
    let mut buf = BytesMut::with_capacity(256);
    buf.resize(128, 0); // current length 128, capacity 256
    let rem = buf.capacity() - buf.len(); // rem = 128
    buf.reserve(rem); // additional = rem, should not allocate
}

#[test]
fn test_bytes_mut_reserve_min_capacity() {
    let mut buf = BytesMut::new();
    buf.resize(10, 0); // current length 10, capacity 10
    let rem = buf.capacity() - buf.len(); // rem = 0
    buf.reserve(rem); // additional = rem, should not allocate
}

