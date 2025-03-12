// Answer 0

#[test]
fn test_shared_clone_min_len() {
    let shared_buf = Box::new(Shared {
        buf: Box::into_raw(Box::new([0u8; 1])) as *mut u8,
        cap: 1,
        ref_cnt: AtomicUsize::new(1),
    });
    let atomic_ptr = AtomicPtr::new(shared_buf as *mut Shared as *mut ());
    let ptr = shared_buf.buf;
    let len = 0;

    unsafe {
        let _bytes = shared_clone(&atomic_ptr, ptr, len);
    }
}

#[test]
fn test_shared_clone_zero_length() {
    let shared_buf = Box::new(Shared {
        buf: Box::into_raw(Box::new([0u8; 10])) as *mut u8,
        cap: 10,
        ref_cnt: AtomicUsize::new(1),
    });
    let atomic_ptr = AtomicPtr::new(shared_buf as *mut Shared as *mut ());
    let ptr = shared_buf.buf;
    let len = 0;

    unsafe {
        let _bytes = shared_clone(&atomic_ptr, ptr, len);
    }
}

#[test]
fn test_shared_clone_full_length() {
    let shared_buf = Box::new(Shared {
        buf: Box::into_raw(Box::new([0u8; 10])) as *mut u8,
        cap: 10,
        ref_cnt: AtomicUsize::new(1),
    });
    let atomic_ptr = AtomicPtr::new(shared_buf as *mut Shared as *mut ());
    let ptr = shared_buf.buf;
    let len = 10;

    unsafe {
        let _bytes = shared_clone(&atomic_ptr, ptr, len);
    }
}

#[test]
fn test_shared_clone_exceeding_length() {
    let shared_buf = Box::new(Shared {
        buf: Box::into_raw(Box::new([0u8; 10])) as *mut u8,
        cap: 10,
        ref_cnt: AtomicUsize::new(1),
    });
    let atomic_ptr = AtomicPtr::new(shared_buf as *mut Shared as *mut ());
    let ptr = shared_buf.buf;
    let len = 11; // Exceeding the allocated size

    unsafe {
        let _bytes = shared_clone(&atomic_ptr, ptr, len);
    }
}

