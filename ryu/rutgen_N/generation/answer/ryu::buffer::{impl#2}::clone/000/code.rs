// Answer 0

#[test]
fn test_clone() {
    struct Buffer;

    impl Buffer {
        fn new() -> Self {
            Buffer
        }

        fn clone(&self) -> Self {
            Buffer::new()
        }
    }

    let buffer = Buffer::new();
    let cloned_buffer = buffer.clone();
    assert_ne!(std::ptr::addr_of!(buffer), std::ptr::addr_of!(cloned_buffer));
}

