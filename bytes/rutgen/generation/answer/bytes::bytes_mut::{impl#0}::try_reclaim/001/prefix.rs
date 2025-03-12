// Answer 0

#[test]
fn test_try_reclaim_capacity_equal_to_remaining() {
    let mut buf = BytesMut::with_capacity(64);
    buf.try_reclaim(64); // Initial state, the capacity should allow reclaim

    assert_eq!(true, buf.try_reclaim(64)); // additional == rem, expect true
}

#[test]
fn test_try_reclaim_capacity_zero() {
    let mut buf = BytesMut::new();
    assert_eq!(true, buf.try_reclaim(0)); // additional == rem (0), expect true
}

#[test]
fn test_try_reclaim_after_extension() {
    let mut buf = BytesMut::with_capacity(64);
    buf.extend_from_slice(b"abcd");
    let split = buf.split(); // Now split is allocated, buf has remaining capacity

    assert_eq!(true, buf.try_reclaim(60)); // additional == rem (60), expect true
}

#[test]
fn test_try_reclaim_with_tight_capacity() {
    let mut buf = BytesMut::new();
    buf.resize(64, 0); // Fill to capacity
    buf.truncate(0); // Clear it

    assert_eq!(true, buf.try_reclaim(64)); // additional == rem (64), expect true
}

