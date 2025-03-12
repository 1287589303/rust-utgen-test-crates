// Answer 0

#[test]
fn test_clone_creates_new_buffer() {
    struct Buffer;

    impl Buffer {
        fn new() -> Self {
            Buffer
        }

        fn clone(&self) -> Self {
            Buffer::new()
        }
    }

    let original_buffer = Buffer::new();
    let cloned_buffer = original_buffer.clone();

    // Here we can assert that the cloned_buffer is a different instance
    assert_ne!(&original_buffer as *const _, &cloned_buffer as *const _);
}

#[test]
fn test_clone_multiple_clone() {
    struct Buffer;

    impl Buffer {
        fn new() -> Self {
            Buffer
        }

        fn clone(&self) -> Self {
            Buffer::new()
        }
    }

    let original_buffer = Buffer::new();
    let cloned_buffer1 = original_buffer.clone();
    let cloned_buffer2 = original_buffer.clone();

    // Ensure that all clones are separate instances
    assert_ne!(&original_buffer as *const _, &cloned_buffer1 as *const _);
    assert_ne!(&original_buffer as *const _, &cloned_buffer2 as *const _);
    assert_ne!(&cloned_buffer1 as *const _, &cloned_buffer2 as *const _);
}

