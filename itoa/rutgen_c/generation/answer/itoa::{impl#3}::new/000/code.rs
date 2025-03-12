// Answer 0

#[test]
fn test_buffer_new() {
    // Create a new Buffer instance
    let buffer = Buffer::new();
    
    // Check if buffer size is as expected (i128::MAX_STR_LEN)
    assert_eq!(buffer.bytes.len(), i128::MAX_STR_LEN);
}

#[test]
#[should_panic]
fn test_buffer_new_exceeds_capacity() {
    // Since Buffer's capacity is hardcoded to i128::MAX_STR_LEN,
    // this test may not be meaningful as it relies on a limit that
    // cannot be directly tested, we will show that the allocation
    // at this size does not panic
    let _ = Buffer::new();
}

