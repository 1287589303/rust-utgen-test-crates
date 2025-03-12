// Answer 0

#[test]
fn test_as_slice_mut_valid_pointer_non_empty() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe { bytes_mut.set_len(5) };
    let slice = bytes_mut.as_slice_mut();
}

#[test]
fn test_as_slice_mut_zero_length() {
    let mut bytes_mut = BytesMut::new();
    unsafe { bytes_mut.set_len(0) };
    let slice = bytes_mut.as_slice_mut();
}

#[test]
fn test_as_slice_mut_full_capacity() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe { bytes_mut.set_len(10) };
    let slice = bytes_mut.as_slice_mut();
}

#[test]
fn test_as_slice_mut_max_length() {
    let mut bytes_mut = BytesMut::with_capacity(usize::MAX >> 1);
    unsafe { bytes_mut.set_len(usize::MAX >> 1) };
    let slice = bytes_mut.as_slice_mut();
}

#[test]
#[should_panic]
fn test_as_slice_mut_exceeding_length() {
    let mut bytes_mut = BytesMut::with_capacity(10);
    unsafe { bytes_mut.set_len(15) }; // Exceeds capacity
    let _slice = bytes_mut.as_slice_mut();
}

