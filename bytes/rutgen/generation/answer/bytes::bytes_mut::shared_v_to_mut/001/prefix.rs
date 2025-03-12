// Answer 0

#[test]
fn test_shared_v_to_mut_unique_case() {
    // Step 1: Create a Vec<u8> and wrap it in Shared
    let vec: Vec<u8> = vec![1, 2, 3, 4, 5];
    let shared = Box::into_raw(Box::new(Shared {
        vec,
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    }));

    // Step 2: Create an AtomicPtr<()>
    let data = AtomicPtr::new(shared.cast());

    // Step 3: Get the pointer to the Vec<u8> and the length
    let ptr = unsafe { (*shared).vec.as_mut_ptr() };
    let len = unsafe { (*shared).vec.len() };

    // Step 4: Call the function under test
    unsafe {
        let result = shared_v_to_mut(&data, ptr, len);
    }
}

#[test]
fn test_shared_v_to_mut_with_full_length() {
    let vec: Vec<u8> = vec![10, 20, 30, 40, 50];
    let shared = Box::into_raw(Box::new(Shared {
        vec,
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    }));

    let data = AtomicPtr::new(shared.cast());
    let ptr = unsafe { (*shared).vec.as_mut_ptr() };
    let len = unsafe { (*shared).vec.len() };

    unsafe {
        let result = shared_v_to_mut(&data, ptr, len);
    }
}

#[test]
fn test_shared_v_to_mut_with_partial_length() {
    let vec: Vec<u8> = vec![100, 200, 300, 400, 500];
    let shared = Box::into_raw(Box::new(Shared {
        vec,
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    }));

    let data = AtomicPtr::new(shared.cast());
    let ptr = unsafe { (*shared).vec.as_mut_ptr() };
    let len = 3; // valid partial length

    unsafe {
        let result = shared_v_to_mut(&data, ptr, len);
    }
}

#[test]
fn test_shared_v_to_mut_with_zero_length() {
    let vec: Vec<u8> = vec![12, 34, 56, 78];
    let shared = Box::into_raw(Box::new(Shared {
        vec,
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(1),
    }));

    let data = AtomicPtr::new(shared.cast());
    let ptr = unsafe { (*shared).vec.as_mut_ptr() };
    let len = 0; // Zero length case

    unsafe {
        let result = shared_v_to_mut(&data, ptr, len);
    }
}

