// Answer 0

#[test]
fn test_owned_drop_impl_non_zero_ref_count_below_max() {
    struct Owned {
        ref_cnt: AtomicUsize,
        drop: unsafe fn(*mut ()),
    }

    let owned_instance = Owned {
        ref_cnt: AtomicUsize::new(usize::MAX >> 1),
        drop: |ptr| { /* Drop implementation */ },
    };

    let ptr: *mut () = &owned_instance as *const _ as *mut ();

    unsafe { owned_drop_impl(ptr) };
}

#[test]
fn test_owned_drop_impl_above_max() {
    struct Owned {
        ref_cnt: AtomicUsize,
        drop: unsafe fn(*mut ()),
    }

    let owned_instance = Owned {
        ref_cnt: AtomicUsize::new(usize::MAX >> 1 + 1),
        drop: |ptr| { /* Drop implementation */ },
    };

    let ptr: *mut () = &owned_instance as *const _ as *mut ();

    unsafe { owned_drop_impl(ptr) };
}

