// Answer 0

#[test]
fn test_advance_mut_with_zero_remaining() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe {
        bytes_mut.set_len(10);
        bytes_mut.advance_mut(0);
    }
}

#[test]
fn test_advance_mut_with_remaining_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(20);
    unsafe {
        bytes_mut.set_len(10);
        bytes_mut.advance_mut(10);
    }
}

#[test]
fn test_advance_mut_with_exact_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(15);
    unsafe {
        bytes_mut.set_len(5);
        bytes_mut.advance_mut(10);
    }
}

