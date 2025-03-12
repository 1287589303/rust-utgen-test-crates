// Answer 0

#[derive(Debug)]
struct Buffer {
    bytes: [std::mem::MaybeUninit<u8>; 39],
}

impl Buffer {
    pub fn new() -> Buffer {
        let bytes = [std::mem::MaybeUninit::<u8>::uninit(); 39];
        Buffer { bytes }
    }
}

#[test]
fn test_buffer_new() {
    let buffer = Buffer::new();
    assert_eq!(buffer.bytes.len(), 39);
}

#[test]
fn test_buffer_bytes_initialization() {
    let buffer = Buffer::new();
    for byte in buffer.bytes.iter() {
        assert!(byte.as_ptr().is_null() || unsafe { byte.assume_init().is_ascii() });
    }
}

