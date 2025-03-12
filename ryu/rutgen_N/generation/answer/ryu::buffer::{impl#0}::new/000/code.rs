// Answer 0

#[test]
fn test_buffer_new() {
    struct Buffer {
        bytes: [std::mem::MaybeUninit<u8>; 24],
    }

    let buffer = Buffer::new();
    assert_eq!(buffer.bytes.len(), 24);
}

impl Buffer {
    pub fn new() -> Self {
        let bytes = [std::mem::MaybeUninit::<u8>::uninit(); 24];
        Buffer { bytes }
    }
}

