// Answer 0

#[test]
fn test_capacity_zero() {
    let b = BytesMut::with_capacity(0);
    let cap = b.capacity();
}

#[test]
fn test_capacity_small() {
    let b = BytesMut::with_capacity(1);
    let cap = b.capacity();
}

#[test]
fn test_capacity_large() {
    let b = BytesMut::with_capacity(1024);
    let cap = b.capacity();
}

#[test]
fn test_capacity_max() {
    let b = BytesMut::with_capacity(usize::MAX);
    let cap = b.capacity();
}

