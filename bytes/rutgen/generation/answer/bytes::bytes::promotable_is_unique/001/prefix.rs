// Answer 0

#[test]
fn test_promotable_is_unique_with_single_reference() {
    // Define the Shared struct directly inside the test for minimal scope
    struct Shared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    // Allocate memory for a Shared struct and create an AtomicPtr pointing to it
    let buf = Box::into_raw(Box::new(0u8)); // Sample buffer
    let shared = Box::into_raw(Box::new(Shared {
        buf,
        cap: 1,
        ref_cnt: AtomicUsize::new(1), // Set ref_cnt to 1
    }));

    // Create an AtomicPtr that points to the Shared struct
    let data = AtomicPtr::new(shared as *mut ());

    // Call the function under test
    let _result = unsafe { promotable_is_unique(&data) };

    // Clean up
    // NOTE: Proper deallocation would require careful handling to ensure safety.
    let _ = data.load(Ordering::Relaxed); // Ensure we load the pointer before cleanup
    unsafe {
        let shared_ptr = Box::from_raw(shared);
        dealloc(shared_ptr.buf, Layout::from_size_align(1, 1).unwrap());
    }
}

