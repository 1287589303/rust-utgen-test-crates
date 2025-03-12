// Answer 0

#[test]
fn test_reserve_inner_arc_with_unique_shared_and_alloc_false() {
    let mut bytes_mut = {
        let shared = Box::into_raw(Box::new(Shared {
            buf: std::ptr::null_mut(),
            cap: 10,
            ref_cnt: AtomicUsize::new(1),
        }));
        BytesMut {
            ptr: NonNull::new(std::ptr::null_mut()).unwrap(),
            len: 5,
            cap: 10,
            data: shared.cast(),
        }
    };

    let additional = 5; // greater than 0
    let allocate = false; // set allocate to false

    let result = bytes_mut.reserve_inner(additional, allocate);
}

#[test]
fn test_reserve_inner_arc_with_unique_shared_and_alloc_false_v_capacity_check() {
    let mut bytes_mut = {
        let shared = Box::into_raw(Box::new(Shared {
            buf: std::ptr::null_mut(),
            cap: 10,
            ref_cnt: AtomicUsize::new(1),
        }));
        BytesMut {
            ptr: NonNull::new(std::ptr::null_mut()).unwrap(),
            len: 5,
            cap: 8, // less than len + additional
            data: shared.cast(),
        }
    };

    let additional = 5; // greater than 0
    let allocate = false; // set allocate to false

    let result = bytes_mut.reserve_inner(additional, allocate);
}

