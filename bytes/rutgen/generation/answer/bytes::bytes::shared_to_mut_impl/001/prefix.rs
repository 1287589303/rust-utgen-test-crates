// Answer 0

#[test]
fn test_shared_to_mut_impl_ref_cnt_greater_than_one() {
    struct MockShared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    let buffer_size = 10;
    let buffer: Vec<u8> = vec![1; buffer_size];
    let mut mock_shared = MockShared {
        buf: buffer.as_mut_ptr(),
        cap: buffer_size,
        ref_cnt: AtomicUsize::new(2), // ref_cnt > 1
    };

    let ptr = mock_shared.buf;
    let len = 5;

    unsafe {
        let result = shared_to_mut_impl(&mut mock_shared as *mut _ as *mut Shared, ptr, len);
    }
}

#[test]
fn test_shared_to_mut_impl_ref_cnt_greater_than_one_with_nonzero_length() {
    struct MockShared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    let buffer_size = 20;
    let buffer: Vec<u8> = vec![2; buffer_size];
    let mut mock_shared = MockShared {
        buf: buffer.as_mut_ptr(),
        cap: buffer_size,
        ref_cnt: AtomicUsize::new(3), // ref_cnt > 1
    };

    let ptr = mock_shared.buf;
    let len = 10;

    unsafe {
        let result = shared_to_mut_impl(&mut mock_shared as *mut _ as *mut Shared, ptr, len);
    }
}

#[test]
fn test_shared_to_mut_impl_ref_cnt_greater_than_one_min_length() {
    struct MockShared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    let buffer_size = 1;
    let buffer: Vec<u8> = vec![3; buffer_size];
    let mut mock_shared = MockShared {
        buf: buffer.as_mut_ptr(),
        cap: buffer_size,
        ref_cnt: AtomicUsize::new(2), // ref_cnt > 1
    };

    let ptr = mock_shared.buf;
    let len = 1;

    unsafe {
        let result = shared_to_mut_impl(&mut mock_shared as *mut _ as *mut Shared, ptr, len);
    }
}

#[test]
fn test_shared_to_mut_impl_ref_cnt_greater_than_one_large_length() {
    struct MockShared {
        buf: *mut u8,
        cap: usize,
        ref_cnt: AtomicUsize,
    }

    let buffer_size = 100;
    let buffer: Vec<u8> = vec![4; buffer_size];
    let mut mock_shared = MockShared {
        buf: buffer.as_mut_ptr(),
        cap: buffer_size,
        ref_cnt: AtomicUsize::new(4), // ref_cnt > 1
    };

    let ptr = mock_shared.buf;
    let len = 50;

    unsafe {
        let result = shared_to_mut_impl(&mut mock_shared as *mut _ as *mut Shared, ptr, len);
    }
}

