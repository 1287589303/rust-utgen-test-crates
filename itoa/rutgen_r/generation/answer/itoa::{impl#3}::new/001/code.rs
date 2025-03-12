// Answer 0

#[test]
fn test_new_buffer() {
    struct Buffer {
        bytes: [core::mem::MaybeUninit<u8>; i128::MAX_STR_LEN],
    }

    impl Buffer {
        pub fn new() -> Self {
            let bytes = [core::mem::MaybeUninit::<u8>::uninit(); i128::MAX_STR_LEN];
            Buffer { bytes }
        }
    }

    let buffer = Buffer::new();
    assert_eq!(buffer.bytes.len(), i128::MAX_STR_LEN);
}

#[test]
fn test_new_buffer_empty() {
    struct Buffer {
        bytes: [core::mem::MaybeUninit<u8>; i128::MAX_STR_LEN],
    }

    impl Buffer {
        pub fn new() -> Self {
            let bytes = [core::mem::MaybeUninit::<u8>::uninit(); i128::MAX_STR_LEN];
            Buffer { bytes }
        }
    }

    let buffer = Buffer::new();
    assert!(buffer.bytes.iter().all(|b| b.as_ptr().is_null()));
}

