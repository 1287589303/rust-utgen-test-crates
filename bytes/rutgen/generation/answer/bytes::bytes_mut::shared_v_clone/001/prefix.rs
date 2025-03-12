// Answer 0

#[test]
fn test_shared_v_clone_valid() {
    let shared = Box::new(Shared {
        vec: vec![1, 2, 3, 4, 5],
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    });

    let ptr = shared.vec.as_ptr();
    let len = shared.vec.len();
    let data = AtomicPtr::new(Box::into_raw(shared) as *mut ());

    unsafe {
        let cloned_bytes = shared_v_clone(&data, ptr, len);
    }
}

#[test]
fn test_shared_v_clone_zero_length() {
    let shared = Box::new(Shared {
        vec: vec![],
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    });

    let ptr = shared.vec.as_ptr();
    let len = shared.vec.len();
    let data = AtomicPtr::new(Box::into_raw(shared) as *mut ());

    unsafe {
        let cloned_bytes = shared_v_clone(&data, ptr, len);
    }
}

#[test]
fn test_shared_v_clone_large_length() {
    let large_vec = vec![0u8; usize::MAX];
    let shared = Box::new(Shared {
        vec: large_vec,
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    });

    let ptr = shared.vec.as_ptr();
    let len = shared.vec.len();
    let data = AtomicPtr::new(Box::into_raw(shared) as *mut ());

    unsafe {
        let cloned_bytes = shared_v_clone(&data, ptr, len);
    }
}

#[should_panic]
fn test_shared_v_clone_invalid_pointer() {
    let shared = Box::new(Shared {
        vec: vec![1, 2, 3],
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    });

    let len = shared.vec.len();
    let data = AtomicPtr::new(Box::into_raw(shared) as *mut ());

    unsafe {
        let cloned_bytes = shared_v_clone(&data, ptr::null(), len);
    }
}

#[should_panic]
fn test_shared_v_clone_exceeding_ref_count() {
    let shared = Box::new(Shared {
        vec: vec![1, 2, 3],
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(usize::MAX),
    });

    let ptr = shared.vec.as_ptr();
    let len = shared.vec.len();
    let data = AtomicPtr::new(Box::into_raw(shared) as *mut ());

    unsafe {
        let cloned_bytes = shared_v_clone(&data, ptr, len);
    }
}

