// Answer 0

#[test]
fn test_owned_drop_impl_with_old_cnt_greater_than_one() {
    struct TestOwnedLifetime {
        ref_cnt: AtomicUsize,
        drop: unsafe fn(*mut ()),
    }

    unsafe extern "C" fn drop_fn(_: *mut ()) {}

    let mut lifetime = TestOwnedLifetime {
        ref_cnt: AtomicUsize::new(2),
        drop: drop_fn,
    };

    let owned: *mut TestOwnedLifetime = &mut lifetime as *mut _;

    owned_drop_impl(owned);
}

#[test]
fn test_owned_drop_impl_with_old_cnt_at_upper_bound() {
    struct TestOwnedLifetime {
        ref_cnt: AtomicUsize,
        drop: unsafe fn(*mut ()),
    }

    unsafe extern "C" fn drop_fn(_: *mut ()) {}

    let mut lifetime = TestOwnedLifetime {
        ref_cnt: AtomicUsize::new(usize::MAX >> 1),
        drop: drop_fn,
    };

    let owned: *mut TestOwnedLifetime = &mut lifetime as *mut _;

    owned_drop_impl(owned);
}

