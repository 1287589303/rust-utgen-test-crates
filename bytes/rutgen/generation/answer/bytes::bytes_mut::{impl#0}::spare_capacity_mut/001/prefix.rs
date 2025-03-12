// Answer 0

#[test]
fn test_spare_capacity_mut_with_non_empty_buffer() {
    let mut buf = BytesMut::with_capacity(10);
    buf.resize(5, 0);
    let spare = buf.spare_capacity_mut();
    assert_eq!(spare.len(), 5);
}

#[test]
fn test_spare_capacity_mut_with_empty_buffer() {
    let mut buf = BytesMut::new();
    buf.reserve(10);
    let spare = buf.spare_capacity_mut();
    assert_eq!(spare.len(), 10);
}

#[test]
fn test_spare_capacity_mut_with_full_capacity() {
    let mut buf = BytesMut::with_capacity(10);
    buf.resize(10, 0);
    let spare = buf.spare_capacity_mut();
    assert_eq!(spare.len(), 0);
}

#[test]
fn test_spare_capacity_mut_with_exceeding_capacity() {
    let mut buf = BytesMut::with_capacity(20);
    buf.resize(15, 0);
    let spare = buf.spare_capacity_mut();
    assert_eq!(spare.len(), 5);
}

#[test]
fn test_spare_capacity_mut_with_zero_capacity() {
    let mut buf = BytesMut::with_capacity(0);
    let spare = buf.spare_capacity_mut();
    assert_eq!(spare.len(), 0);
}

