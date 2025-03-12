// Answer 0

#[test]
fn test_remaining_mut_zero_length() {
    let bytes_mut = BytesMut::with_capacity(10);
    unsafe {
        bytes_mut.set_len(0);
    }
    let _result = bytes_mut.remaining_mut();
}

#[test]
fn test_remaining_mut_mid_length() {
    let bytes_mut = BytesMut::with_capacity(100);
    unsafe {
        bytes_mut.set_len(50);
    }
    let _result = bytes_mut.remaining_mut();
}

#[test]
fn test_remaining_mut_max_length() {
    let bytes_mut = BytesMut::with_capacity(usize::MAX);
    unsafe {
        bytes_mut.set_len(usize::MAX);
    }
    let _result = bytes_mut.remaining_mut();
}

