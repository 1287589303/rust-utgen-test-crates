// Answer 0

#[test]
fn test_default_buffer() {
    struct Buffer {
        data: Vec<u8>,
    }

    impl Buffer {
        fn new() -> Buffer {
            Buffer { data: Vec::new() }
        }
    }

    let buffer = Buffer::default();
    assert_eq!(buffer.data.len(), 0);
}

