// Answer 0

#[test]
fn test_split_off_at_zero() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.extend_from_slice(&[1, 2, 3, 4, 5]);
    let other = bytes_mut.split_off(0);
}

#[test]
fn test_split_off_at_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.extend_from_slice(&[1, 2, 3, 4, 5]);
    let other = bytes_mut.split_off(5);
}

#[test]
fn test_split_off_below_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.extend_from_slice(&[1, 2, 3, 4, 5]);
    let other = bytes_mut.split_off(4);
}

