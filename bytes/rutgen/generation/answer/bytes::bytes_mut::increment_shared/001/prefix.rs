// Answer 0

#[test]
fn test_increment_shared_overflow() {
    let shared_instance = Shared {
        vec: Vec::new(),
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(isize::MAX as usize),
    };
    let ptr = &shared_instance as *const Shared as *mut Shared;

    unsafe {
        increment_shared(ptr);
    }
}

#[test]
#[should_panic]
fn test_increment_shared_exceeds_limit() {
    let shared_instance = Shared {
        vec: Vec::new(),
        original_capacity_repr: 0,
        ref_count: AtomicUsize::new(isize::MAX as usize),
    };
    let ptr = &shared_instance as *const Shared as *mut Shared;

    unsafe {
        increment_shared(ptr);
    }
}

