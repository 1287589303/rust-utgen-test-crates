// Answer 0

#[test]
fn test_write_str_exact_capacity() {
    struct TestBufMut {
        bytes: BytesMut,
    }

    impl TestBufMut {
        fn new(capacity: usize) -> Self {
            let vec = vec![0u8; capacity];
            Self {
                bytes: BytesMut {
                    ptr: NonNull::new(vec.as_mut_ptr()).unwrap(),
                    len: 0,
                    cap: capacity,
                    data: ptr::null_mut(),
                },
            }
        }
    }

    let capacity = 17; // MAX_ORIGINAL_CAPACITY_WIDTH
    let mut test_buf = TestBufMut::new(capacity);
    let s = "test_string"; // Length of 11
    test_buf.bytes.len = 11; // Emulate buffer's used length equal to string length
    unsafe {
        test_buf.bytes.write_str(s).unwrap();
    }
}

#[test]
fn test_write_str_min_length() {
    struct TestBufMut {
        bytes: BytesMut,
    }

    impl TestBufMut {
        fn new(capacity: usize) -> Self {
            let vec = vec![0u8; capacity];
            Self {
                bytes: BytesMut {
                    ptr: NonNull::new(vec.as_mut_ptr()).unwrap(),
                    len: 0,
                    cap: capacity,
                    data: ptr::null_mut(),
                },
            }
        }
    }

    let capacity = 10; // MIN_ORIGINAL_CAPACITY_WIDTH
    let mut test_buf = TestBufMut::new(capacity);
    let s = "abc"; // Length of 3
    test_buf.bytes.len = 3; // Emulate buffer's used length equal to string length
    unsafe {
        test_buf.bytes.write_str(s).unwrap();
    }
}

#[test]
fn test_write_str_empty() {
    struct TestBufMut {
        bytes: BytesMut,
    }

    impl TestBufMut {
        fn new(capacity: usize) -> Self {
            let vec = vec![0u8; capacity];
            Self {
                bytes: BytesMut {
                    ptr: NonNull::new(vec.as_mut_ptr()).unwrap(),
                    len: 0,
                    cap: capacity,
                    data: ptr::null_mut(),
                },
            }
        }
    }

    let capacity = 15; // Within the defined maximum capacity
    let mut test_buf = TestBufMut::new(capacity);
    let s = ""; // Empty string
    test_buf.bytes.len = 0; // Emulate buffer's used length for empty string
    unsafe {
        test_buf.bytes.write_str(s).unwrap();
    }
}

