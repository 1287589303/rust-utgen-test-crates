// Answer 0

#[test]
fn test_release_shared_ref_count_greater_than_one() {
    let shared = Box::new(Shared {
        vec: vec![1, 2, 3],
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(2),
    });

    let ptr = Box::into_raw(shared);
    unsafe {
        release_shared(ptr);
    }
}

#[test]
fn test_release_shared_ref_count_equals_two() {
    let shared = Box::new(Shared {
        vec: vec![4, 5, 6],
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(2),
    });

    let ptr = Box::into_raw(shared);
    unsafe {
        release_shared(ptr);
    }
}

#[test]
fn test_release_shared_ref_count_initially_set_to_two() {
    let shared = Box::new(Shared {
        vec: vec![7, 8, 9],
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(2),
    });

    let ptr = Box::into_raw(shared);
    unsafe {
        release_shared(ptr);
    }
}

