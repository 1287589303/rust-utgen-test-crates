// Answer 0

#[test]
fn test_free_boxed_slice_valid() {
    let len = 10;
    let buffer = unsafe { alloc::alloc::alloc(Layout::from_size_align(len, 1).unwrap()) };
    let offset = unsafe { buffer.add(2) }; // Ensure offset is within bounds
    let dealloc_len = 5; // Length to deallocate

    unsafe { free_boxed_slice(buffer, offset, dealloc_len) };
}

#[test]
fn test_free_boxed_slice_full_length() {
    let len = 10;
    let buffer = unsafe { alloc::alloc::alloc(Layout::from_size_align(len, 1).unwrap()) };
    let offset = unsafe { buffer }; // Offset at the beginning of the buffer
    let dealloc_len = len; // Deallocate full length

    unsafe { free_boxed_slice(buffer, offset, dealloc_len) };
}

#[test]
fn test_free_boxed_slice_zero_length() {
    let len = 10;
    let buffer = unsafe { alloc::alloc::alloc(Layout::from_size_align(len, 1).unwrap()) };
    let offset = unsafe { buffer.add(1) }; // Valid offset within buffer
    let dealloc_len = 0; // Deallocate length of zero

    unsafe { free_boxed_slice(buffer, offset, dealloc_len) };
}

#[test]
#[should_panic]
fn test_free_boxed_slice_invalid_offset() {
    let len = 10;
    let buffer = unsafe { alloc::alloc::alloc(Layout::from_size_align(len, 1).unwrap()) };
    let offset = unsafe { buffer.add(len + 1) }; // Invalid offset outside the buffer
    let dealloc_len = 1; // Arbitrary length

    unsafe { free_boxed_slice(buffer, offset, dealloc_len) };
}

#[test]
#[should_panic]
fn test_free_boxed_slice_exceeding_length() {
    let len = 10;
    let buffer = unsafe { alloc::alloc::alloc(Layout::from_size_align(len, 1).unwrap()) };
    let offset = unsafe { buffer.add(5) }; // Valid offset
    let dealloc_len = 6; // Length exceeds available memory

    unsafe { free_boxed_slice(buffer, offset, dealloc_len) };
}

