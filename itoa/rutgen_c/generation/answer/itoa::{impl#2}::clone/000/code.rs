// Answer 0

#[test]
fn test_buffer_clone() {
    // Instantiate the Buffer using the provided new method
    let original_buffer = Buffer::new();
    
    // Clone the buffer
    let cloned_buffer = original_buffer.clone();
    
    // Ensure that the cloned buffer is not the same instance as the original
    assert!(!ptr::eq(&original_buffer, &cloned_buffer));
    
    // Ensure that after cloning, the cloned buffer is initialized properly
    let original_bytes: &[MaybeUninit<u8>] = unsafe {
        slice::from_raw_parts(original_buffer.bytes.as_ptr() as *const MaybeUninit<u8>, original_buffer.bytes.len())
    };
    
    let cloned_bytes: &[MaybeUninit<u8>] = unsafe {
        slice::from_raw_parts(cloned_buffer.bytes.as_ptr() as *const MaybeUninit<u8>, cloned_buffer.bytes.len())
    };

    for (orig, clone) in original_bytes.iter().zip(cloned_bytes.iter()) {
        assert_eq!(orig, clone);
    }
}

