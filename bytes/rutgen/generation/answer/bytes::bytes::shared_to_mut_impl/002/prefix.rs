// Answer 0

#[test]
fn test_shared_to_mut_impl_ref_count_one_and_non_empty_buffer() {
    let shared = Box::into_raw(Box::new(Shared {
        buf: Box::into_raw(Box::new([1u8, 2u8, 3u8])),
        cap: 3,
        ref_cnt: AtomicUsize::new(1),
    }));
    let ptr = unsafe { (*shared).buf };
    let len = 3;

    let _result = unsafe { shared_to_mut_impl(shared, ptr, len) };
}

#[test]
fn test_shared_to_mut_impl_ref_count_one_with_zero_length() {
    let shared = Box::into_raw(Box::new(Shared {
        buf: Box::into_raw(Box::new([0u8; 10])),
        cap: 10,
        ref_cnt: AtomicUsize::new(1),
    }));
    let ptr = unsafe { (*shared).buf };
    let len = 0;

    let _result = unsafe { shared_to_mut_impl(shared, ptr, len) };
}

#[test]
fn test_shared_to_mut_impl_ref_count_one_with_partial_length() {
    let shared = Box::into_raw(Box::new(Shared {
        buf: Box::into_raw(Box::new([10u8, 20u8, 30u8, 40u8, 50u8])),
        cap: 5,
        ref_cnt: AtomicUsize::new(1),
    }));
    let ptr = unsafe { (*shared).buf };
    let len = 2;

    let _result = unsafe { shared_to_mut_impl(shared, ptr, len) };
}

