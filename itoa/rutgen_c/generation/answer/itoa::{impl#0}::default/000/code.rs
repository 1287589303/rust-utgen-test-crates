// Answer 0

#[test]
fn test_buffer_default() {
    let buffer = Buffer::default();
    // Ensure that the buffer is initialized correctly
    assert_eq!(core::mem::size_of_val(&buffer.bytes), core::mem::size_of::<[MaybeUninit<u8>; i128::MAX_STR_LEN]>());
}

#[test]
fn test_buffer_new() {
    let buffer = Buffer::new();
    // Ensure that the buffer is initialized correctly
    assert_eq!(core::mem::size_of_val(&buffer.bytes), core::mem::size_of::<[MaybeUninit<u8>; i128::MAX_STR_LEN]>());
}

