// Answer 0

#[test]
fn test_release_shared_with_ref_count_greater_than_one() {
    let shared = Box::new(Shared {
        buf: Core::ptr::null_mut(),
        cap: 1024,
        ref_cnt: AtomicUsize::new(2), // Initial count set to 2
    });
    
    let ptr = Box::into_raw(shared);
    
    unsafe {
        release_shared(ptr);
    }
}

#[test]
fn test_release_shared_multiple_references() {
    let shared = Box::new(Shared {
        buf: Core::ptr::null_mut(),
        cap: 512,
        ref_cnt: AtomicUsize::new(3), // Initial count set to 3
    });
    
    let ptr = Box::into_raw(shared);
    
    unsafe {
        release_shared(ptr);
    }
}

#[test]
fn test_release_shared_boundary_ref_count() {
    let shared = Box::new(Shared {
        buf: Core::ptr::null_mut(),
        cap: 256,
        ref_cnt: AtomicUsize::new(2), // Setting it to 2 to just meet the condition
    });
    
    let ptr = Box::into_raw(shared);
    
    unsafe {
        release_shared(ptr);
    }
}

#[test]
fn test_release_shared_with_large_capacity() {
    let shared = Box::new(Shared {
        buf: Core::ptr::null_mut(),
        cap: usize::MAX, // Using maximum capacity to check behavior
        ref_cnt: AtomicUsize::new(4), // Sufficiently high ref count
    });

    let ptr = Box::into_raw(shared);

    unsafe {
        release_shared(ptr);
    }
}

