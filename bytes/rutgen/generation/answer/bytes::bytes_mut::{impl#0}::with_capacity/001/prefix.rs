// Answer 0

#[test]
fn test_with_capacity_zero() {
    let bytes = BytesMut::with_capacity(0);
    let len = bytes.len();
    let cap = bytes.capacity();
}

#[test]
fn test_with_capacity_one() {
    let bytes = BytesMut::with_capacity(1);
    let len = bytes.len();
    let cap = bytes.capacity();
}

#[test]
fn test_with_capacity_ten() {
    let bytes = BytesMut::with_capacity(10);
    let len = bytes.len();
    let cap = bytes.capacity();
}

#[test]
fn test_with_capacity_seventeen() {
    let bytes = BytesMut::with_capacity(17);
    let len = bytes.len();
    let cap = bytes.capacity();
}

#[test]
fn test_with_capacity_sixty_four() {
    let bytes = BytesMut::with_capacity(64);
    let len = bytes.len();
    let cap = bytes.capacity();
}

#[test]
fn test_with_capacity_usize_max() {
    let bytes = BytesMut::with_capacity(usize::MAX);
    let len = bytes.len();
    let cap = bytes.capacity();
}

