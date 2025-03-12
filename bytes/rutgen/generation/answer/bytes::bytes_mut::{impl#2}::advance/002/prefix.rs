// Answer 0

#[test]
fn test_advance_exceed_remaining() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    bytes_mut.resize(5, 0); // Set length to 5, remaining should be 5
    let cnt = bytes_mut.remaining() + 1; // Exceeds remaining

    unsafe {
        bytes_mut.advance(cnt);
    }
}

#[test]
#[should_panic]
fn test_advance_zero() {
    let mut bytes_mut = BytesMut::new(); // Initial length is 0, remaining is 0
    let cnt = 0; // Zero count

    unsafe {
        bytes_mut.advance(cnt);
    }
}

#[test]
#[should_panic]
fn test_advance_exceed_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(5);
    bytes_mut.resize(3, 0); // Set length to 3, remaining should be 3
    let cnt = bytes_mut.capacity() + 1; // Exceeds capacity

    unsafe {
        bytes_mut.advance(cnt);
    }
}

