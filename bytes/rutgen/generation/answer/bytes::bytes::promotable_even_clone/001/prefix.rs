// Answer 0

#[test]
fn test_promotable_even_clone_arc() {
    use core::ptr::{null_mut, NonNull};
    
    // Create a shared object with KIND_ARC
    let shared = Box::new(crate::Shared {
        buf: null_mut(),
        cap: 0,
        ref_cnt: AtomicUsize::new(1),
    });
    let shared_ptr = Box::into_raw(shared);

    let atomic_ptr = AtomicPtr::new(shared_ptr as *mut ());

    // Pointer to valid memory location
    let ptr = NonNull::new(0x1234 as *const u8).unwrap();
    let len = 10;

    // Call the function under test
    unsafe {
        let _ = promotable_even_clone(&atomic_ptr, ptr.as_ptr(), len);
    }
}

#[test]
fn test_promotable_even_clone_arc_non_null_ptr() {
    use core::ptr::{null_mut, NonNull};

    // Create a shared object with KIND_ARC
    let shared = Box::new(crate::Shared {
        buf: null_mut(),
        cap: 20,
        ref_cnt: AtomicUsize::new(1),
    });
    let shared_ptr = Box::into_raw(shared);

    let atomic_ptr = AtomicPtr::new(shared_ptr as *mut ());

    // Pointer to valid memory location
    let ptr = NonNull::new(0x5678 as *const u8).unwrap();
    let len = 5;

    // Call the function under test
    unsafe {
        let _ = promotable_even_clone(&atomic_ptr, ptr.as_ptr(), len);
    }
}

#[test]
fn test_promotable_even_clone_arc_non_zero_length() {
    use core::ptr::{null_mut, NonNull};

    // Create a shared object with KIND_ARC
    let shared = Box::new(crate::Shared {
        buf: null_mut(),
        cap: 100,
        ref_cnt: AtomicUsize::new(1),
    });
    let shared_ptr = Box::into_raw(shared);

    let atomic_ptr = AtomicPtr::new(shared_ptr as *mut ());

    // Pointer to valid memory location
    let ptr = NonNull::new(0x9abc as *const u8).unwrap();
    let len = 1; // Minimum non-zero length

    // Call the function under test
    unsafe {
        let _ = promotable_even_clone(&atomic_ptr, ptr.as_ptr(), len);
    }
}

