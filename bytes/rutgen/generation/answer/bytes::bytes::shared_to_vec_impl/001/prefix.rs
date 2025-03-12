// Answer 0

#[test]
fn test_shared_to_vec_impl_unique_ref_count() {
    let cap = 10;
    let buf = Box::into_raw(Box::new(vec![0u8; cap]));
    let shared = Box::into_raw(Box::new(Shared {
        buf,
        cap,
        ref_cnt: AtomicUsize::new(1),
    }));

    let data: *const u8 = vec![1u8, 2u8, 3u8].as_ptr();
    let len: usize = 3;

    unsafe {
        let result = shared_to_vec_impl(shared, data, len);
    }
}

#[test]
fn test_shared_to_vec_impl_multiple_calls() {
    let cap = 20;
    let buf = Box::into_raw(Box::new(vec![0u8; cap]));
    let shared = Box::into_raw(Box::new(Shared {
        buf,
        cap,
        ref_cnt: AtomicUsize::new(1),
    }));

    let data: *const u8 = vec![4u8, 5u8, 6u8, 7u8, 8u8].as_ptr();
    let len: usize = 5;

    unsafe {
        let result1 = shared_to_vec_impl(shared, data, len);
        let result2 = shared_to_vec_impl(shared, data, len);
    }
}

#[test]
fn test_shared_to_vec_impl_boundary_length() {
    let cap = 5;
    let buf = Box::into_raw(Box::new(vec![0u8; cap]));
    let shared = Box::into_raw(Box::new(Shared {
        buf,
        cap,
        ref_cnt: AtomicUsize::new(1),
    }));

    let data: *const u8 = vec![9u8, 10u8, 11u8, 12u8, 13u8].as_ptr();
    let len: usize = 5;

    unsafe {
        let result = shared_to_vec_impl(shared, data, len);
    }
}

#[test]
fn test_shared_to_vec_impl_small_length() {
    let cap = 10;
    let buf = Box::into_raw(Box::new(vec![0u8; cap]));
    let shared = Box::into_raw(Box::new(Shared {
        buf,
        cap,
        ref_cnt: AtomicUsize::new(1),
    }));

    let data: *const u8 = vec![14u8, 15u8].as_ptr();
    let len: usize = 2;

    unsafe {
        let result = shared_to_vec_impl(shared, data, len);
    }
}

