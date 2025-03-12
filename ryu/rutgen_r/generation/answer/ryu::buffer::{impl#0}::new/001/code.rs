// Answer 0

#[test]
fn test_buffer_new() {
    struct Buffer {
        bytes: [std::mem::MaybeUninit<u8>; 24],
    }

    let buffer = Buffer::new();
    let expected_bytes = [std::mem::MaybeUninit::<u8>::uninit(); 24];

    assert_eq!(buffer.bytes, expected_bytes);
}

impl Buffer {
    pub fn new() -> Self {
        let bytes = [std::mem::MaybeUninit::<u8>::uninit(); 24];
        Buffer { bytes }
    }
}

