// Answer 0

#[test]
fn test_buffer_new_initializes_bytes() {
    let buffer = Buffer::new();
    let bytes: &[MaybeUninit<u8>] = &buffer.bytes;
    assert_eq!(bytes.len(), i128::MAX_STR_LEN as usize);
    for byte in bytes.iter() {
        assert_eq!(byte, &MaybeUninit::<u8>::uninit());
    }
}

#[test]
fn test_buffer_new_returns_correct_type() {
    let buffer: Buffer = Buffer::new();
    // Just checking the type with a dummy assertion.
    let _: &Buffer = &buffer;
}

