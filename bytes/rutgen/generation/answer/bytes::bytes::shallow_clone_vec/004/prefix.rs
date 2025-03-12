// Answer 0

#[test]
unsafe fn test_shallow_clone_vec_success() {
    let atom = AtomicPtr::new(ptr::null_mut());
    let mut buffer: Vec<u8> = Vec::with_capacity(10);
    buffer.extend_from_slice(&[1, 2, 3, 4, 5]);
    let buf_ptr = buffer.as_mut_ptr();
    let offset = buf_ptr.add(2); // Non-null and within bounds
    let len = 3; // Greater than 0 and less than or equal to capacity

    // Ensure the shared pointer is null initially
    assert!(atom.load(Ordering::Relaxed).is_null());

    let result = shallow_clone_vec(&atom, buf_ptr as _, buf_ptr, offset, len);
}

#[test]
unsafe fn test_shallow_clone_vec_concurrent_clone() {
    let atom = AtomicPtr::new(ptr::null_mut());
    let mut buffer: Vec<u8> = Vec::with_capacity(10);
    buffer.extend_from_slice(&[1, 2, 3, 4, 5]);
    let buf_ptr = buffer.as_mut_ptr();
    let offset = buf_ptr.add(2); // Non-null and within bounds
    let len = 3; // Greater than 0 and less than or equal to capacity

    // Ensure the shared pointer is null initially
    assert!(atom.load(Ordering::Relaxed).is_null());

    // Create two clones concurrently
    let handle1 = std::thread::spawn(move || {
        shallow_clone_vec(&atom, buf_ptr as _, buf_ptr, offset, len)
    });

    let handle2 = std::thread::spawn(move || {
        shallow_clone_vec(&atom, buf_ptr as _, buf_ptr, offset, len)
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

#[test]
unsafe fn test_shallow_clone_vec_zero_length() {
    let atom = AtomicPtr::new(ptr::null_mut());
    let mut buffer: Vec<u8> = Vec::with_capacity(10);
    let buf_ptr = buffer.as_mut_ptr();
    let offset = buf_ptr; // Non-null and within bounds
    let len = 0; // Less than or equal to capacity, but edge case for zero length

    // Ensure the shared pointer is null initially
    assert!(atom.load(Ordering::Relaxed).is_null());

    let result = shallow_clone_vec(&atom, buf_ptr as _, buf_ptr, offset, len);
}

