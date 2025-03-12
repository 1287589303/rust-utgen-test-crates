// Answer 0

#[test]
#[should_panic]
fn test_advance_mut_above_remaining() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe {
        bytes_mut.advance_mut(bytes_mut.capacity() + 1); // cnt > remaining
    }
}

#[test]
#[should_panic]
fn test_advance_mut_above_remaining_boundary() {
    let mut bytes_mut = BytesMut::with_capacity(5);
    bytes_mut.resize(5, 0); // Fill to capacity
    unsafe {
        bytes_mut.advance_mut(6); // cnt > remaining
    }
}

